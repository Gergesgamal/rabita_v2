-- backend/migrations/20261027000023_components.down.sql

SET search_path = app;

DROP TABLE IF EXISTS puck_component_approval;
DROP TABLE IF EXISTS puck_component;
