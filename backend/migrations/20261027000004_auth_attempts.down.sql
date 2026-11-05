-- backend/migrations/20261027000004_auth_attempts.down.sql

SET search_path = app;

DROP TABLE IF EXISTS idempotency_key;
DROP TABLE IF EXISTS attempt_token;
