-- backend/migrations/20261027000012_orders.down.sql

SET search_path = app;

DROP TABLE IF EXISTS order_status_history;
DROP TABLE IF EXISTS stock_reserve;
DROP TABLE IF EXISTS order_item;
DROP TABLE IF EXISTS "order";
DROP TABLE IF EXISTS payment_status_customization;
DROP TABLE IF EXISTS payment_status_default;
DROP TABLE IF EXISTS order_status_customization;
DROP TABLE IF EXISTS order_status_default;
