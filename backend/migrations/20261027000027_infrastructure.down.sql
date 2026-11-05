-- backend/migrations/20261027000027_infrastructure.down.sql

SET search_path = app;

DROP TABLE IF EXISTS notification_log;
DROP TABLE IF EXISTS health_check_log;
