-- backend/migrations/20261027000022_pages.down.sql

SET search_path = app;

DROP TABLE IF EXISTS page_preview_token;
DROP TABLE IF EXISTS puck_page_publish;
DROP TABLE IF EXISTS puck_page_revision;
DROP TABLE IF EXISTS puck_page;
