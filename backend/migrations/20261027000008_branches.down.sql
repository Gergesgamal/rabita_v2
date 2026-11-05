-- backend/migrations/20261027000008_branches.down.sql

SET search_path = app;

DROP TABLE IF EXISTS branch_working_hours;
DROP TABLE IF EXISTS branch;
