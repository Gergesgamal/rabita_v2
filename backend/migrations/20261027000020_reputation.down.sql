-- backend/migrations/20261027000020_reputation.down.sql

SET search_path = app;

DROP TABLE IF EXISTS reputation_event;
DROP TABLE IF EXISTS reputation;
