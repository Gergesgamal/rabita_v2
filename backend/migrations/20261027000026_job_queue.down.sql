-- backend/migrations/20261027000026_job_queue.down.sql

SET search_path = app;

DROP TABLE IF EXISTS outbox_attempt;
DROP TABLE IF EXISTS outbox;
