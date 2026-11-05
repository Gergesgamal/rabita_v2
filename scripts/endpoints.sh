#!/usr/bin/env bash
# Endpoint harness: discovers route declarations in source, probes the running backend against
# scripts/endpoints.txt, and reports PASS / FAIL / MISSING / WARN. A 404 is always MISSING, never PASS.
# Usage: ./scripts/endpoints.sh   (BASE=http://host:port to override)
set -uo pipefail
cd "$(dirname "$0")/.."
BASE=${BASE:-http://localhost:3000}
MANIFEST=scripts/endpoints.txt
PASS=0; FAIL=0; MISSING=0; WARN=0
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT
line() { printf '  %-8s %s\n' "$1" "$2"; }

echo "== Route declarations found in source (not necessarily mounted)"
found=$(grep -rnoE '\.(route|nest)\("[^"]+"' backend/src frontend/src 2>/dev/null)
if [ -n "$found" ]; then echo "$found" | sed 's/^/  /'; else echo "  (none)"; fi

echo "== Webhook references in source"
grep -rniE 'webhook' backend/src frontend/src 2>/dev/null | sed 's/^/  /' || true
[ -z "$(grep -rliE 'webhook' backend/src frontend/src 2>/dev/null)" ] && echo "  (none)"

echo "== Reachability"
if [ "$(curl -s -o /dev/null -w '%{http_code}' "$BASE/health")" != 200 ]; then
  echo "  backend not answering at $BASE. Start it with ./scripts/up.sh"; exit 1
fi
echo "  backend answering at $BASE"

echo "== Spec endpoints"
while read -r kind method path body allowed; do
  case "$kind" in ''|\#*) continue ;; esac
  if [ "$body" = "-" ]; then
    code=$(curl -s -o "$tmp" -w '%{http_code}' -X "$method" "$BASE$path")
  else
    code=$(curl -s -o "$tmp" -w '%{http_code}' -X "$method" -H 'content-type: application/json' --data "$body" "$BASE$path")
  fi
  label="$method $path -> $code"
  if grep -qiE 'panicked|backtrace|stack trace' "$tmp"; then
    line FAIL "$label  (stack trace leaked in response body)"; FAIL=$((FAIL+1))
  elif [ "$code" -ge 500 ] 2>/dev/null; then
    line FAIL "$label  (server error)"; FAIL=$((FAIL+1))
  elif [ "$kind" = protected ] && [ "$code" = 200 ]; then
    line FAIL "$label  (protected endpoint served without authentication)"; FAIL=$((FAIL+1))
  elif [ "$code" = 404 ]; then
    line MISSING "$label  (in the blueprint, not mounted on the server)"; MISSING=$((MISSING+1))
  elif echo " $allowed " | grep -q " $code "; then
    line PASS "$label"; PASS=$((PASS+1))
  else
    line WARN "$label  (expected one of: $allowed)"; WARN=$((WARN+1))
  fi
done < "$MANIFEST"

echo
echo "== Summary: $PASS pass, $FAIL fail, $MISSING missing, $WARN warn"
echo "  FAIL = security or server error (fix first)."
echo "  MISSING = the blueprint defines it and the server does not mount it yet."
echo "  Webhooks: the blueprint defines none yet, so the webhook probe only checks that unsigned requests are refused."
[ "$FAIL" -eq 0 ] && exit 0 || exit 1
