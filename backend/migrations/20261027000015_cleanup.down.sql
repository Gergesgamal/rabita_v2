-- backend/migrations/20261027000015_cleanup.down.sql

SET search_path = app;

DROP TABLE IF EXISTS cleanup_run;
DROP TABLE IF EXISTS cleanup_rule;
