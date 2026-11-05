#!/usr/bin/env bash
# Probe the running backend: endpoint behavior, security headers, CORS, rate limiting, body size, benchmark.
# Usage: ./scripts/check.sh   (BASE=http://host:port to override)
set -uo pipefail
BASE=${BASE:-http://localhost:3000}
PASS=0; WARN=0
ok()   { printf '  [ok]   %s\n' "$1"; PASS=$((PASS+1)); }
warn() { printf '  [WARN] %s\n' "$1"; WARN=$((WARN+1)); }
info() { printf '  [info] %s\n' "$1"; }
code() { curl -s -o /dev/null -w '%{http_code}' "$@"; }

echo "== Reachability"
if [ "$(code "$BASE/health")" = 200 ]; then ok "GET /health -> 200"; else warn "backend not answering at $BASE. Start it with ./scripts/up.sh"; echo; echo "Stopped."; exit 1; fi

echo "== Endpoint behavior"
for p in /health /ready; do
  c=$(code "$BASE$p"); if [ "$c" = 200 ]; then ok "GET $p -> $c"; else warn "GET $p -> $c"; fi
done
for p in /api/v1/auth/login /api/v1/auth/logout; do
  info "POST $p -> $(code -X POST "$BASE$p")   (404 means not mounted)"
done
info "GET /does-not-exist -> $(code "$BASE/does-not-exist")   (expect 404)"
info "DELETE /health -> $(code -X DELETE "$BASE/health")   (expect 405)"

echo "== Security headers on /health"
hdrs=$(curl -sI "$BASE/health" | tr 'A-Z' 'a-z')
for h in x-content-type-options x-frame-options content-security-policy referrer-policy strict-transport-security; do
  if echo "$hdrs" | grep -q "^$h:"; then ok "$h present"; else warn "$h missing (strict-transport-security is expected over plain HTTP in dev)"; fi
done

echo "== CORS"
if curl -sI -H 'Origin: https://evil.example' "$BASE/health" | tr 'A-Z' 'a-z' | grep -q '^access-control-allow-origin'; then
  warn "CORS allows an arbitrary origin (evil.example)"
else
  ok "CORS does not reflect an arbitrary origin"
fi

echo "== Request body limit"
big=$(head -c 2097152 /dev/zero | tr '\0' 'a')
c=$(curl -s -o /dev/null -w '%{http_code}' -X POST -H 'content-type: application/json' --data-binary "$big" "$BASE/api/v1/auth/login")
info "2 MiB POST -> $c   (expect 413 once body limits exist; 404 means the route isn't mounted)"

echo "== Rate limiting (200 requests, 20 parallel)"
rl=$(seq 1 200 | xargs -P 20 -I{} curl -s -o /dev/null -w '%{http_code}\n' "$BASE/health" | sort | uniq -c)
echo "$rl" | sed 's/^/  /'
if echo "$rl" | grep -q ' 429$'; then ok "429 returned under burst"; else warn "no rate limiting observed"; fi

echo "== Performance (20 s, 50 connections each)"
if command -v oha >/dev/null 2>&1; then
  for p in /health /ready; do
    echo "  --- $p"
    oha -z 20s -c 50 --no-tui "$BASE$p" | grep -E 'Requests/sec|Success rate|99.00%' | sed 's/^/  /'
  done
else
  warn "oha not installed. Install with: cargo install oha"
fi

echo
echo "== Summary: $PASS ok, $WARN warning(s)"
echo "  This covers only the endpoints the backend serves today. Auth routes are placeholders,"
echo "  /ready does not query Postgres or Valkey, and there are no authenticated endpoints to test."
echo "  Re-run after the real routes are restored so the results cover the actual API."
