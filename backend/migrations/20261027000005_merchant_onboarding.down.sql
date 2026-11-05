-- backend/migrations/20261027000005_merchant_onboarding.down.sql

SET search_path = app;

DROP TABLE IF EXISTS merchant_application;
