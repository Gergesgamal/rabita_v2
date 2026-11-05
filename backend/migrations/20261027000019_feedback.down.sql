-- backend/migrations/20261027000019_feedback.down.sql

SET search_path = app;

DROP TABLE IF EXISTS feedback_item;
