-- backend/migrations/20261027000014_promotions.down.sql

SET search_path = app;

DROP TABLE IF EXISTS bundle_partial_application;
DROP TABLE IF EXISTS bundle_item;
DROP TABLE IF EXISTS bundle;
DROP TABLE IF EXISTS coupon;
DROP TABLE IF EXISTS discount;
