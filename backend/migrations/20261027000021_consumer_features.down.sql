-- backend/migrations/20261027000021_consumer_features.down.sql

SET search_path = app;

DROP TABLE IF EXISTS consumer_consent;
DROP TABLE IF EXISTS recently_viewed;
DROP TABLE IF EXISTS back_in_stock_alert;
DROP TABLE IF EXISTS wishlist;
DROP TABLE IF EXISTS address_book;
