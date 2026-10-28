# 1. Start services
docker-compose up -d

# 2. Create database
sqlx database create

# 3. Run migrations (no-op in Wave 1)
sqlx migrate run

# 4. Build
cargo build

# 5. Run server
cargo run -p backend &

# 6. Test health endpoint
curl localhost:3000/health
# Expected: 200 {"status":"ok","service":"rabita-backend"}

# 7. Test readiness endpoint
curl localhost:3000/ready
# Expected: 200 {"ready":true,"dependencies":{"postgres":"ok","valkey":"ok"}}

# 8. Stop Valkey and test fail-closed readiness
docker-compose stop valkey
curl localhost:3000/ready
# Expected: 503 with "valkey" in failing array

# 9. Restart Valkey
docker-compose start valkey
