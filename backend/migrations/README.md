# Rabita Schema Migrations

This directory contains PostgreSQL migrations for the Rabita commerce platform schema.

## Migration Overview

All 27 migrations are listed below with their tables and Wave dependency:

| Migration | Tables | Wave |
|---|---|---|
| 0001_schema_metadata | schema_metadata | 2a |
| 0002_identity | account, account_role, session, webauthn_credential, totp_secret, recovery_code, consumer | 2a |
| 0003_audit | audit_log | 2a |
| 0004_auth_attempts | attempt_token, idempotency_key | 2a |
| 0005_merchant_onboarding | merchant_application | 2a |
| 0006_delegation | permission_catalog, account_permission, approval_request, execution_token | 2a |
| 0007_stores | store, store_settings, tier2_reserved_slug | 2a |
| 0008_branches | branch, branch_working_hours | 2a |
| 0009_catalog | category, product, variant, product_image, branch_product | 2a |
| 0010_media | media, media_quota | 2a |
| 0011_shipping | shipping_policy, delivery_slot_config, shipping_rule | 2a |
| 0012_orders | order_status_default, order_status_customization, payment_status_default, payment_status_customization, order, order_item, stock_reserve, order_status_history | 2a |
| 0013_cart_reconciliation | cart_audit | 2a |
| 0014_promotions | discount, coupon, bundle, bundle_item, bundle_partial_application | 2a |
| 0015_cleanup | cleanup_rule, cleanup_run | 2b |
| 0016_refunds | refund_request, refund_decision | 2b |
| 0017_support | ticket, ticket_reply, ticket_attachment, ticket_last_viewed | 2b |
| 0018_jobs | job_post, job_application, job_chat_thread, job_chat_message, employee_relation | 2b |
| 0019_feedback | feedback_item | 2b |
| 0020_reputation | reputation, reputation_event | 2b |
| 0021_consumer_features | address_book, wishlist, back_in_stock_alert, recently_viewed, consumer_consent | 2b |
| 0022_pages | puck_page, puck_page_revision, puck_page_publish, page_preview_token | 2b |
| 0023_components | puck_component, puck_component_approval | 2b |
| 0024_branding | branding, invoice_counter | 2b |
| 0025_cms | cms_page, cms_page_revision | 2b |
| 0026_job_queue | outbox, outbox_attempt | 2b |
| 0027_infrastructure | health_check_log, notification_log | 2b |

## Key Schema Design Principles

- **UUIDs**: All primary keys are `UUID PRIMARY KEY DEFAULT gen_random_uuid()`
- **Timestamps**: All tables have `created_at` and `updated_at TIMESTAMPTZ DEFAULT now()`
- **Soft Deletes**: User-visible tables include `deleted_at TIMESTAMPTZ`
- **Money**: Stored as `BIGINT` (Egyptian piastres, 100 piastres = 1 EGP)
- **Durations**: Stored as `BIGINT` (seconds) with `CHECK (duration >= 0)` per D-081
- **Foreign Keys**: All FKs include explicit `ON DELETE` action
- **Indexes**: Created on tenant columns (`store_id`, `branch_id`), FKs, and commonly-filtered columns
- **RLS**: Policies on multi-tenant tables use `current_setting('app.current_context_id')::uuid`
- **Comments**: All tables and non-obvious columns include COMMENT ON for clarity

## Running Migrations

```bash
# Create database
sqlx database create

# Run all pending migrations
sqlx migrate run

# Verify idempotency (no-op on second run)
sqlx migrate run

# Generate SQLx offline metadata
cargo sqlx prepare

# Check for drift
cargo sqlx prepare --check
```

## Adding New Migrations

```bash
# Create a new migration
sqlx migrate add -r my_feature_name

# This creates:
# backend/migrations/yyyyMMddHHmmss_my_feature_name.up.sql
# backend/migrations/yyyyMMddHHmmss_my_feature_name.down.sql

# Add your SQL, then run to test
sqlx migrate run
cargo sqlx prepare
```

## Multi-Tenancy

All migrations enforce tenant isolation through:

1. **Application-layer filtering**: Every query filters by `store_id` or `context_id`
2. **RLS policies**: Critical tables (orders, media, consumers) have RLS enabled
3. **Index placement**: Indexes on `store_id` and `branch_id` ensure efficient tenant scoping

## Audit and Compliance

- `audit_log` is append-only with hash-chaining for tamper detection
- `idempotency_key` prevents duplicate processing per D-130
- `outbox` implements reliable async delivery per D-084
- All sensitive data (`passwords`, `secrets`, `tokens`, `OTPs`) excluded from logs by design

## Data Retention

- `cleanup_rule` defines per-category retention policies
- `deleted_at` enables soft-deletes with recovery window
- `audit_log` is permanent for critical events
- `outbox_attempt` logs all retry attempts
