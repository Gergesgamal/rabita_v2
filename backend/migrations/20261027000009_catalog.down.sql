-- backend/migrations/20261027000009_catalog.down.sql

SET search_path = app;

DROP TABLE IF EXISTS branch_product;
DROP TABLE IF EXISTS product_image;
DROP TABLE IF EXISTS variant;
DROP TABLE IF EXISTS product;
DROP TABLE IF EXISTS category;
