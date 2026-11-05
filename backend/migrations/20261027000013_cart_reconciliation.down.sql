-- backend/migrations/20261027000013_cart_reconciliation.down.sql

SET search_path = app;

DROP TABLE IF EXISTS cart_audit;
