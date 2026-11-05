-- backend/migrations/20261027000001_schema_metadata.down.sql

DROP TABLE IF EXISTS app.schema_metadata;
DROP EXTENSION IF EXISTS unaccent;
DROP EXTENSION IF EXISTS pg_trgm;
DROP EXTENSION IF EXISTS pgcrypto;
DROP EXTENSION IF EXISTS "uuid-ossp";
DROP SCHEMA IF EXISTS app;
