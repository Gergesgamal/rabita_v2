-- backend/migrations/20261027000017_support.down.sql

SET search_path = app;

DROP TABLE IF EXISTS ticket_last_viewed;
DROP TABLE IF EXISTS ticket_attachment;
DROP TABLE IF EXISTS ticket_reply;
DROP TABLE IF EXISTS ticket;
