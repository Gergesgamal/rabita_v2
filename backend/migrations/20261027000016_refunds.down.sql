-- backend/migrations/20261027000016_refunds.down.sql

SET search_path = app;

DROP TABLE IF EXISTS refund_decision;
DROP TABLE IF EXISTS refund_request;
