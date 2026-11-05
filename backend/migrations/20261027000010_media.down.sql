-- backend/migrations/20261027000010_media.down.sql

SET search_path = app;

DROP TABLE IF EXISTS media_quota;
DROP TABLE IF EXISTS media;
