-- backend/migrations/20261027000007_stores.up.sql

SET LOCAL search_path = app, public;

-- Store table per D-021
CREATE TABLE store (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_account_id UUID NOT NULL REFERENCES account(id) ON DELETE RESTRICT,
    slug TEXT UNIQUE NOT NULL,
    business_name TEXT NOT NULL,
    description TEXT,
    contact_phone TEXT,
    contact_email TEXT,
    is_published BOOLEAN DEFAULT false,
    is_suspended BOOLEAN DEFAULT false,
    suspension_reason TEXT,
    store_timezone TEXT DEFAULT 'Africa/Cairo', -- D-127
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    
    CHECK (slug ~ '^[a-z0-9-]{3,63}$'),
    CHECK (slug NOT LIKE '-%' AND slug NOT LIKE '%-%'),
    CHECK (contact_email IS NULL OR contact_email ~ '^[^@]+@[^@]+\.[^@]+$')
);

COMMENT ON TABLE store IS 'Merchant storefronts per D-021; slug is immutable, lowercase ASCII, 3-63 chars';
COMMENT ON COLUMN store.slug IS 'Immutable, normalized via NFC + IDN/Punycode to ASCII, 3-63 alphanumeric + hyphens';
COMMENT ON COLUMN store.is_published IS 'Separate from account approval; controls public visibility';

CREATE INDEX idx_store_owner_account_id ON store(owner_account_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_store_slug ON store(slug) WHERE deleted_at IS NULL;
CREATE INDEX idx_store_is_published ON store(is_published) WHERE deleted_at IS NULL;
CREATE INDEX idx_store_created_at ON store(created_at DESC);

-- Add FK to consumer now that store exists
ALTER TABLE consumer ADD CONSTRAINT fk_consumer_store_id FOREIGN KEY (store_id) REFERENCES store(id) ON DELETE CASCADE;
CREATE INDEX idx_consumer_store_id_new ON consumer(store_id) WHERE deleted_at IS NULL;

-- Store settings per D-039/D-050
CREATE TABLE store_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL UNIQUE REFERENCES store(id) ON DELETE CASCADE,
    max_employees SMALLINT DEFAULT 10,
    onboarding_mode TEXT NOT NULL DEFAULT 'invite_only', -- 'open', 'review', 'invite_only'
    guest_checkout_enabled BOOLEAN DEFAULT true,
    auto_refund_cutoff_days SMALLINT DEFAULT 3,
    ticket_lifetime_days SMALLINT DEFAULT 14,
    cancellation_window_hours BIGINT, -- NULL = no self-service cancellation, stored as seconds per D-081
    allow_merchant_cancellation BOOLEAN DEFAULT true,
    tax_rate_bp SMALLINT DEFAULT 1400, -- 14% VAT default per D-118
    invoice_numbering TEXT DEFAULT 'sequential_per_store', -- 'sequential_per_store', 'sequential_platform', 'none'
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (max_employees > 0),
    CHECK (onboarding_mode IN ('open', 'review', 'invite_only')),
    CHECK (auto_refund_cutoff_days > 0),
    CHECK (ticket_lifetime_days >= 10 AND ticket_lifetime_days <= 90),
    CHECK (tax_rate_bp >= 0 AND tax_rate_bp <= 5000)
);

COMMENT ON TABLE store_settings IS 'Per-store configuration for commerce rules, onboarding, and behavior per D-050';
COMMENT ON COLUMN store_settings.cancellation_window_hours IS 'NULL = no self-service; stored as BIGINT seconds per D-081';
COMMENT ON COLUMN store_settings.tax_rate_bp IS 'Basis points; 1400 = 14% default per D-118, merchant can change';

CREATE INDEX idx_store_settings_store_id ON store_settings(store_id);

-- Tier 2 reserved slugs (admin-managed)
CREATE TABLE tier2_reserved_slug (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug TEXT UNIQUE NOT NULL,
    reason TEXT, -- 'business_reservation', 'trademark', 'campaign', etc.
    reserved_by_id UUID NOT NULL REFERENCES account(id) ON DELETE SET NULL,
    reserved_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (slug ~ '^[a-z0-9-]{3,63}$')
);

COMMENT ON TABLE tier2_reserved_slug IS 'Admin-managed reserved slugs per D-022, reported for collisions but does not invalidate existing stores';

CREATE INDEX idx_tier2_reserved_slug_slug ON tier2_reserved_slug(slug);
