-- backend/migrations/20261027000007_stores.down.sql

SET search_path = app;

DROP TABLE IF EXISTS tier2_reserved_slug;
DROP TABLE IF EXISTS store_settings;
DROP TABLE IF EXISTS store CASCADE;
