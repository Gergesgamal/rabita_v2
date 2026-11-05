-- backend/migrations/20261027000006_delegation.down.sql

SET search_path = app;

DROP TABLE IF EXISTS execution_token;
DROP TABLE IF EXISTS approval_request;
DROP TABLE IF EXISTS account_permission;
DROP TABLE IF EXISTS permission_catalog;
