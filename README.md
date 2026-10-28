# Rabita Backend

Rabita is an Egypt-first commerce operating platform. This repository contains the Rust/Axum backend.

## Prerequisites

- **Rust 1.75+**: Install via [rustup](https://rustup.rs/)
- **Docker & Docker Compose**: For PostgreSQL, Valkey, and MinIO
- **sqlx-cli**: `cargo install sqlx-cli --no-default-features --features native-tls,postgres`
- **Cargo and Tokio runtime**

## Quick Start

### 1. Clone and Setup

```bash
git clone <repo>
cd rabita
cp .env.example .env
```

### 2. Start Services

```bash
docker-compose up -d
```

Verify containers are healthy:
```bash
docker-compose ps
```

Expected: All services show `healthy` status.

### 3. Create Database

```bash
sqlx database create
```

### 4. Run Migrations (None yet in Wave 1)

```bash
sqlx migrate run
```

Output: `0 migrations applied` in Wave 1.

### 5. Prepare SQLx Offline Metadata

This generates `.sqlx/` metadata for compile-time query checking. Required before building Wave 3+:

```bash
cargo sqlx prepare
```

Expected: `.sqlx/` directory created (empty in Wave 1).

### 6. Build and Run

```bash
cargo build
cargo run -p backend
```

Expected output:

Starting Rabita backend in development environment
Database and cache connections established
Server listening on 127.0.0.1:3000


### 7. Test Health Endpoints

In another terminal:

```bash
# Liveness check (no dependencies)
curl http://localhost:3000/health
# Expected: 200 OK {"status":"ok","service":"rabita-backend"}

# Readiness check (dependencies required)
curl http://localhost:3000/ready
# Expected: 200 OK {"ready":true,"dependencies":{"postgres":"ok","valkey":"ok"}}
```

### 8. Stop Services

```bash
docker-compose stop
cargo kill  # or Ctrl+C in the terminal running cargo run
```

## Workflow for Future Waves

### After Adding Migrations (Wave 2a)

```bash
sqlx migrate run           # Apply all migrations
cargo sqlx prepare        # Regenerate .sqlx/ metadata
cargo build               # Compile with new queries
cargo test                # Run test suite
```

### After Adding New Endpoints (Wave 3+)

```bash
cargo sqlx prepare        # Must run after any SQL changes
cargo build
cargo test
```

### Running Tests

```bash
# Unit and integration tests
cargo test

# With output
cargo test -- --nocapture

# Specific module
cargo test auth::
```

## Environment Variables

Copy `.env.example` to `.env` and customize:

| Variable | Purpose | Default |
|---|---|---|
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://rabita:...@127.0.0.1:5432/rabita` |
| `CACHE_REDIS_URL` | Valkey (Redis-compatible) URL | `redis://127.0.0.1:6379` |
| `AUTH_JWT_SECRET` | Secret for JWT signing | (required, minimum 32 chars) |
| `AUTH_WEBAUTHN_RP_ID` | WebAuthn Relying Party ID | `localhost` (development) |
| `APP_LOG_LEVEL` | Tracing log level | `info` |

## Docker Services

### PostgreSQL (Port 5432)
- User: `rabita`
- Password: `dev-password-change-in-production`
- Database: `rabita`
- Volume: `postgres_data`

### Valkey (Port 6379)
- Append-only file enabled
- Volume: `valkey_data`

### MinIO (Ports 9000, 9001)
- Access Key: `minioadmin`
- Secret Key: `minioadmin`
- Web UI: http://localhost:9001
- Volume: `minio_data`

## Debugging

### Check Container Logs

```bash
docker-compose logs postgres
docker-compose logs valkey
docker-compose logs minio
```

### Verify Database Connection

```bash
PGPASSWORD=dev-password-change-in-production psql -h 127.0.0.1 -U rabita -d rabita -c "SELECT 1;"
```

### Check Valkey

```bash
docker exec rabita-valkey valkey-cli ping
# Expected: PONG
```

## Architecture

- **Framework**: Axum 0.7+ (Tokio async)
- **Database**: PostgreSQL 16 with SQLx for compile-time safety
- **Cache**: Valkey 8 (Redis-compatible)
- **Storage**: MinIO locally, AWS S3 in production
- **Error Handling**: RFC 9457 Problem Details

## Wave Delivery Structure

1. **Wave 1 (Foundation)**: Server, pools, health checks ✓
2. **Wave 2a–2b**: Database schema (27 migrations)
3. **Wave 3a–3c**: Authentication, authorization, onboarding
4. **Wave 4**: Stores, catalog, media, shipping
5. **Wave 5**: Cart, checkout, orders, refunds
6. **Wave 6**: Support, jobs, feedback, analytics
7. **Wave 7a–7b**: Leptos frontend
8. **Wave 8**: Puck editor, tests, CI

Total: ~250 files delivered in 12 waves over ~7 hours.

## Production Deployment

See `docs/DEPLOYMENT.md` (delivered in Wave 8) for:
- AWS infrastructure setup
- RDS Aurora configuration
- CloudFront distribution
- Route53 DNS
- AWS WAF configuration
- IAM roles and policies

## License

Proprietary — Rabita Inc.
