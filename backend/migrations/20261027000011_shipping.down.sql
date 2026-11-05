-- backend/migrations/20261027000011_shipping.down.sql

SET search_path = app;

DROP TABLE IF EXISTS shipping_rule;
DROP TABLE IF EXISTS delivery_slot_config;
DROP TABLE IF EXISTS shipping_policy;
