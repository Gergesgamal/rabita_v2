-- backend/migrations/20261027000018_jobs.down.sql

SET search_path = app;

DROP TABLE IF EXISTS job_chat_message;
DROP TABLE IF EXISTS job_chat_thread;
DROP TABLE IF EXISTS job_application;
DROP TABLE IF EXISTS job_post;
DROP TABLE IF EXISTS employee_relation;
