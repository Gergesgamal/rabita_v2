-- backend/migrations/20261027000024_branding.down.sql

SET search_path = app;

DROP TABLE IF EXISTS invoice_counter;
DROP TABLE IF EXISTS branding;
