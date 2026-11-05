# Development Setup

## Local Environment

### 1. Clone & Install

```bash
git clone https://github.com/your-org/rabita
cd rabita

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres
```

### 2. Start Services

```bash
docker-compose up -d
```

### 3. Run Migrations

```bash
source .env
sqlx migrate run
```

### 4. Start Development Servers

**Backend** (in one terminal):
```bash
cargo run --package rabita-backend
# Backend running on http://localhost:3000
```

**Frontend** (in another terminal):
```bash
cd frontend
cargo leptos serve
# Frontend running on http://localhost:3001
```

### 5. Access Local Services

- **Merchant Dashboard**: http://localhost:3001
- **Admin**: http://localhost:3001/admin
- **API**: http://localhost:3000/api/v1/
- **Postgres**: localhost:5432 (user: rabita, password: rabita_dev)
- **Valkey**: localhost:6379
- **MinIO Console**: http://localhost:9001 (user: minioadmin, password: minioadmin)

## Debugging

### Backend Debug Logs

```bash
RUST_LOG=debug cargo run --package rabita-backend
```

### Database Query Logging

```bash
SQLX_LOGGING=true cargo run --package rabita-backend
```

### Browser DevTools

Frontend runs with source maps enabled for Leptos debugging.

## Code Style

### Rust

- Run `cargo fmt` before commit
- Run `cargo clippy` for linting
- Pre-commit hook:

```bash
#!/bin/bash
cargo fmt --all
cargo clippy --all -- -D warnings
git add -A
```

## Database Schema Changes

### Add a Column

1. **Create migration**:
   ```bash
   sqlx migrate add -r my_migration
   ```

2. **Write SQL** (up and down):
   ```sql
   -- up
   ALTER TABLE app.my_table ADD COLUMN new_column TEXT;

   -- down
   ALTER TABLE app.my_table DROP COLUMN new_column;
   ```

3. **Run locally**:
   ```bash
   sqlx migrate run
   ```

4. **Test rollback**:
   ```bash
   sqlx migrate revert
   sqlx migrate run
   ```

5. **Commit**:
   ```bash
   git add migrations/
   git commit -m "chore: add new_column to my_table"
   ```

## Useful Commands

```bash
# Format code
cargo fmt --all

# Lint
cargo clippy --all -- -D warnings

# Run tests
cargo test

# Run specific test
cargo test test_name -- --nocapture

# Check dependencies
cargo outdated
cargo audit

# Generate API docs
cargo doc --no-deps --open
```

## Troubleshooting

### "Failed to connect to Postgres"

```bash
# Check Postgres is running
docker-compose ps

# Restart if needed
docker-compose restart postgres

# Verify connection
psql -U rabita -d rabita -h localhost
```

### "Valkey connection error"

```bash
# Check Valkey is running
docker-compose ps

# Test connection
redis-cli -h localhost -p 6379 ping
```

### "Migration already exists"

```bash
# Reset migrations (local only!)
sqlx database reset
sqlx migrate run
```
