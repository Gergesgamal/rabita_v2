-- backend/migrations/20261027000025_cms.down.sql

SET search_path = app;

DROP TABLE IF EXISTS cms_page_revision;
DROP TABLE IF EXISTS cms_page;
