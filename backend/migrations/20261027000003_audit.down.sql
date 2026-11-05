-- backend/migrations/20261027000003_audit.down.sql

SET search_path = app;

DROP TABLE IF EXISTS audit_log;
