#!/usr/bin/env bash
# Preflight, start, and verify every local component of Rabita.
# Usage: ./scripts/start-all.sh
set -uo pipefail
cd "$(dirname "$0")/.."

MISSING=0
ok()   { printf '  [OK]       %s\n' "$1"; }
warn() { printf '  [WARN]     %s\n               fix: %s\n' "$1" "$2"; }
bad()  { printf '  [MISSING]  %s\n               fix: %s\n' "$1" "$2"; MISSING=$((MISSING+1)); }
hdr()  { printf '\n== %s ==\n' "$1"; }
have() { command -v "$1" >/dev/null 2>&1; }

hdr "1. Tools"
have docker && ok "docker" || bad "docker" "install Docker (https://docs.docker.com/engine/install/) and start it"
docker compose version >/dev/null 2>&1 && ok "docker compose v2" || bad "docker compose v2" "sudo apt install docker-compose-plugin"
have cargo && ok "cargo ($(cargo --version 2>/dev/null | cut -d' ' -f2))" || bad "cargo" "curl https://sh.rustup.rs -sSf | sh   then: source ~/.cargo/env"
have curl && ok "curl" || bad "curl" "sudo apt install curl"
have psql && ok "psql (optional)" || warn "psql not installed (optional)" "sudo apt install postgresql-client, or use: docker compose exec postgres psql -U rabita -d rabita"
have oha && ok "oha (optional, for bench.sh)" || warn "oha not installed (optional)" "cargo install oha"

hdr "2. Docker daemon"
if docker info >/dev/null 2>&1; then
  ok "daemon reachable"
else
  bad "Docker daemon not reachable" "start Docker Desktop, or: sudo systemctl start docker && sudo usermod -aG docker \$USER (then log out and back in)"
fi

hdr "3. Project files"
for f in docker-compose.yml Dockerfile .dockerignore Cargo.toml Cargo.lock backend/Cargo.toml frontend/Cargo.toml; do
  [ -f "$f" ] && ok "$f" || bad "$f missing" "restore it from git or your backup (run cargo check once to regenerate Cargo.lock)"
done

hdr "4. Configuration (.env)"
if [ -f .env ]; then
  ok ".env present"
  set -a; . ./.env; set +a
  for k in DATABASE_URL VALKEY_URL LEPTOS_OUTPUT_NAME LEPTOS_SITE_ADDR; do
    [ -n "${!k:-}" ] && ok "$k set" || bad "$k not set in .env" "add $k=... to .env (see the .env block from the setup message)"
  done
else
  bad ".env missing" "create .env in the project root with DATABASE_URL, VALKEY_URL, LEPTOS_OUTPUT_NAME, LEPTOS_SITE_ADDR"
fi

hdr "5. Ports"
for p in 5432 6379 9000 9001 3000 3001; do
  if (exec 3<>"/dev/tcp/127.0.0.1/$p") 2>/dev/null; then
    if docker ps --format '{{.Ports}}' | grep -q ":$p->"; then
      ok "port $p (used by a running container)"
    else
      bad "port $p already in use by another program" "find it: sudo ss -ltnp | grep :$p   then stop that program"
    fi
  else
    ok "port $p free"
  fi
done

if [ "$MISSING" -gt 0 ]; then
  echo; echo "Preflight found $MISSING missing requirement(s). Fix the [MISSING] items above, then re-run."
  exit 1
fi

svc_id()     { docker compose ps -q "$1" 2>/dev/null; }
svc_health() { local id; id=$(svc_id "$1"); [ -n "$id" ] && docker inspect -f '{{if .State.Health}}{{.State.Health.Status}}{{else}}{{.State.Status}}{{end}}' "$id" 2>/dev/null; }
healthy()    { [ "$(svc_health "$1")" = healthy ]; }
wait_until() { # name, fix-hint, timeout-in-2s-steps, command...
  local name=$1 hint=$2 t=$3; shift 3
  for _ in $(seq 1 "$t"); do
    if "$@" >/dev/null 2>&1; then ok "$name ready"; return 0; fi
    sleep 2
  done
  bad "$name not ready after $((t*2))s" "$hint"
  return 1
}

hdr "6. Infrastructure (Docker)"
docker compose up -d postgres valkey minio || bad "docker compose up (postgres, valkey, minio) failed" "docker compose logs postgres valkey minio --tail 50"
wait_until "PostgreSQL" "docker compose logs postgres --tail 50" 60 healthy postgres
wait_until "Valkey" "docker compose logs valkey --tail 50" 60 healthy valkey
wait_until "MinIO" "docker compose logs minio --tail 50" 60 curl -fsS http://localhost:9000/minio/health/live

hdr "7. Backend (Docker)"
if docker compose up -d --build backend; then
  wait_until "backend /health" "docker compose logs backend --tail 80" 120 curl -fsS http://localhost:3000/health
else
  bad "backend build or start failed" "docker compose logs backend --tail 80; if disk is full: docker system prune -f"
fi

hdr "8. Frontend (cargo, on host)"
mkdir -p logs
nohup cargo run -p rabita-frontend > logs/frontend.log 2>&1 &
echo "  building and starting frontend (log: logs/frontend.log) ..."
wait_until "frontend /health" "tail -80 logs/frontend.log" 300 curl -fsS "http://$LEPTOS_SITE_ADDR/health"

hdr "9. Connectivity checks"
docker compose exec -T postgres psql -U rabita -d rabita -tAc 'select 1' >/dev/null 2>&1 \
  && ok "postgres accepts a query" \
  || bad "postgres query failed" "DATABASE_URL must match docker-compose.yml (user rabita, password rabita_dev, db rabita)"
docker compose exec -T valkey valkey-cli ping 2>/dev/null | grep -q PONG \
  && ok "valkey answers PING" \
  || bad "valkey ping failed" "docker compose logs valkey --tail 50"
curl -fsS http://localhost:3000/ready >/dev/null 2>&1 \
  && ok "backend /ready responds" \
  || bad "backend /ready failed" "docker compose logs backend --tail 80"

hdr "10. AWS"
ok "not needed for local testing (storage is local MinIO, minioadmin/minioadmin)"
echo "               AWS IAM is needed only when pointing at real S3. You would then need an IAM user"
echo "               with s3:GetObject, PutObject, DeleteObject, ListBucket on your bucket, an access key,"
echo "               and AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_REGION, S3_BUCKET in .env."

hdr "Summary"
echo "  backend   http://localhost:3000/health"
echo "  frontend  http://$LEPTOS_SITE_ADDR/health"
echo "  postgres  localhost:5432  (rabita / rabita_dev / rabita)"
echo "  valkey    localhost:6379"
echo "  minio     http://localhost:9001  (minioadmin / minioadmin)"
echo
if [ "$MISSING" -eq 0 ]; then echo "All checks passed."; else echo "$MISSING check(s) failed. See the [MISSING] lines above."; fi
exit "$MISSING"
