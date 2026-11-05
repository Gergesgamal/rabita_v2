-- backend/migrations/20261027000024_branding.up.sql

SET LOCAL search_path = app, public;

-- Store branding per D-099
CREATE TABLE branding (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL UNIQUE REFERENCES store(id) ON DELETE CASCADE,
    logo_media_id UUID, -- REFERENCES media table
    favicon_media_id UUID,
    accent_color TEXT, -- Validated hex color per D-099
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (accent_color IS NULL OR accent_color ~ '^#[0-9a-f]{6}$')
);

COMMENT ON TABLE branding IS 'Store branding per D-099 (MVP: logo, favicon, accent color only)';

CREATE UNIQUE INDEX idx_branding_store_id ON branding(store_id);

-- Invoice counter per D-122
CREATE TABLE invoice_counter (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL UNIQUE REFERENCES store(id) ON DELETE CASCADE,
    numbering_scheme TEXT NOT NULL DEFAULT 'sequential_per_store', -- 'sequential_per_store', 'sequential_platform', 'none'
    next_sequence BIGINT DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (numbering_scheme IN ('sequential_per_store', 'sequential_platform', 'none')),
    CHECK (next_sequence > 0)
);

COMMENT ON TABLE invoice_counter IS 'Invoice number generation per D-122; idempotent per order';

CREATE UNIQUE INDEX idx_invoice_counter_store_id ON invoice_counter(store_id);
