#!/usr/bin/env bash
# Rabita local bring-up: checks requirements, starts infra, backend, frontend, smoke-tests.
set -uo pipefail
cd "$(dirname "$0")/.."
mkdir -p logs
MISSING=0
ok()   { printf '  [ok]   %s\n' "$1"; }
miss() { printf '  [MISS] %s\n         how to fix: %s\n' "$1" "$2"; MISSING=$((MISSING+1)); }
note() { printf '  [note] %s\n' "$1"; }
step() { printf '\n== %s\n' "$1"; }
gate() { if [ "$MISSING" -gt 0 ]; then echo; echo "Stopped. Fix the [MISS] item(s) above, then re-run ./scripts/up.sh"; exit 1; fi; }

step "1. Host tools"
if command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1; then ok "docker daemon running"; else miss "docker daemon not reachable" "sudo systemctl start docker (or open Docker Desktop)"; fi
if docker compose version >/dev/null 2>&1; then ok "docker compose v2"; else miss "docker compose v2 missing" "sudo apt install docker-compose-plugin"; fi
if command -v cargo >/dev/null 2>&1; then ok "cargo $(cargo --version | awk '{print $2}')"; else miss "cargo missing" "install Rust: curl https://sh.rustup.rs -sSf | sh"; fi
if command -v curl >/dev/null 2>&1; then ok "curl"; else miss "curl missing" "sudo apt install curl"; fi
gate

step "2. Config (.env)"
if [ -f .env ]; then ok ".env present"; else miss ".env missing" "create .env in the project root with DATABASE_URL, VALKEY_URL, RUST_LOG, LEPTOS_OUTPUT_NAME, LEPTOS_SITE_ADDR, LEPTOS_SITE_ROOT"; fi
gate
set -a; . ./.env; set +a
for v in DATABASE_URL VALKEY_URL LEPTOS_OUTPUT_NAME LEPTOS_SITE_ADDR; do
  if [ -n "${!v:-}" ]; then ok "$v set"; else miss "$v not set in .env" "add $v=... to .env"; fi
done
gate

step "3. Infrastructure containers"
if docker compose up -d postgres valkey minio > logs/compose.log 2>&1; then ok "postgres, valkey, minio started"; else miss "containers failed to start" "see logs/compose.log; if disk full: docker system prune -f; if port busy: ss -ltnp | grep -E '5432|6379|9000'"; fi
gate
wait_health() {
  local cid st
  for _ in $(seq 1 60); do
    cid=$(docker compose ps -q "$1")
    st=$(docker inspect -f '{{if .State.Health}}{{.State.Health.Status}}{{else}}{{.State.Status}}{{end}}' "$cid" 2>/dev/null)
    if [ "$st" = healthy ] || [ "$st" = running ]; then return 0; fi
    sleep 2
  done
  return 1
}
for s in postgres valkey minio; do
  if wait_health "$s"; then ok "$s ready"; else miss "$s not ready after 2 min" "docker compose logs $s"; fi
done
gate

step "4. Service checks"
if docker run --rm --network host postgres:16-alpine psql "$DATABASE_URL" -tAc 'select 1' > logs/pg.log 2>&1; then
  ok "postgres accepts DATABASE_URL"
elif grep -q 28P01 logs/pg.log; then
  miss "postgres password mismatch for DATABASE_URL user" "docker compose down && docker volume rm rabita_postgres_data && docker compose up -d postgres   (wipes local DB volume; it is empty so far)"
else
  miss "postgres unreachable at DATABASE_URL" "check host/port/user in DATABASE_URL; details in logs/pg.log"
fi
if [ "$(docker compose exec -T valkey valkey-cli ping 2>/dev/null | tr -d '\r')" = PONG ]; then ok "valkey answers PING"; else miss "valkey not answering" "docker compose logs valkey"; fi
if curl -fsS http://127.0.0.1:9000/minio/health/live >/dev/null 2>&1; then ok "minio health live on :9000"; else miss "minio not answering on :9000" "docker compose logs minio"; fi
if ls backend/migrations/*.sql >/dev/null 2>&1; then ok "migrations present"; else note "no SQL migrations in backend/migrations yet, so the database has no tables. Fine for the current stub backend."; fi
note "AWS/IAM: not required for local testing. No code calls AWS yet; MinIO (minioadmin / minioadmin) stands in for S3."
gate

step "5. Backend"
if cargo build --release -p backend --bin backend > logs/build-backend.log 2>&1; then ok "backend builds"; else miss "backend build failed" "tail -40 logs/build-backend.log"; fi
gate
pkill -f 'target/release/backend' 2>/dev/null || true; sleep 1
nohup target/release/backend > logs/backend.log 2>&1 &
for _ in $(seq 1 30); do curl -fsS http://127.0.0.1:3000/health >/dev/null 2>&1 && break; sleep 1; done
if curl -fsS http://127.0.0.1:3000/health >/dev/null 2>&1; then ok "backend listening on :3000"; else miss "backend did not start" "tail -20 logs/backend.log"; fi
gate

step "6. Frontend"
if cargo build -p rabita-frontend > logs/build-frontend.log 2>&1; then ok "frontend builds"; else miss "frontend build failed" "tail -40 logs/build-frontend.log"; fi
gate
pkill -f 'target/debug/rabita-frontend' 2>/dev/null || true; sleep 1
nohup target/debug/rabita-frontend > logs/frontend.log 2>&1 &
for _ in $(seq 1 30); do curl -fsS "http://$LEPTOS_SITE_ADDR/health" >/dev/null 2>&1 && break; sleep 1; done
if curl -fsS "http://$LEPTOS_SITE_ADDR/health" >/dev/null 2>&1; then ok "frontend listening on $LEPTOS_SITE_ADDR"; else miss "frontend did not start" "tail -20 logs/frontend.log"; fi
gate

step "Summary"
echo "  All requirements met."
echo "  Backend:  http://localhost:3000   (curl http://localhost:3000/health)"
echo "  Frontend: http://$LEPTOS_SITE_ADDR   (curl http://$LEPTOS_SITE_ADDR/health)"
echo "  Benchmark: ./scripts/bench.sh"
echo "  Stop everything: pkill -f target/release/backend; pkill -f target/debug/rabita-frontend; docker compose down"
