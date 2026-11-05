-- backend/migrations/20261027000001_schema_metadata.up.sql

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "unaccent";

-- Create schema
CREATE SCHEMA IF NOT EXISTS app;
SET LOCAL search_path = app, public;

-- Enable RLS
ALTER DEFAULT PRIVILEGES GRANT USAGE ON SCHEMAS TO PUBLIC;

-- Metadata table for tracking schema version
CREATE TABLE schema_metadata (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    schema_version TEXT NOT NULL,
    applied_at TIMESTAMPTZ DEFAULT now(),
    created_at TIMESTAMPTZ DEFAULT now()
);

COMMENT ON TABLE schema_metadata IS 'Tracks schema version and migration history';

-- Insert initial version
INSERT INTO schema_metadata (schema_version) VALUES ('0001');
