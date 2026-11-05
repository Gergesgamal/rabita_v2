-- backend/migrations/20261027000002_identity.down.sql

SET search_path = app;

DROP TABLE IF EXISTS consumer;
DROP TABLE IF EXISTS recovery_code;
DROP TABLE IF EXISTS totp_secret;
DROP TABLE IF EXISTS webauthn_credential;
DROP TABLE IF EXISTS session;
DROP TABLE IF EXISTS account_role;
DROP TABLE IF EXISTS account;
