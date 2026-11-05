-- backend/migrations/20261027000011_shipping.up.sql

SET LOCAL search_path = app, public;

-- Shipping policies (generic terms/conditions)
CREATE TABLE shipping_policy (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    content TEXT, -- Sanitized HTML
    is_published BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    
    UNIQUE (store_id, title)
);

COMMENT ON TABLE shipping_policy IS 'Merchant-authored shipping policies (returns, general terms) per D-062';

CREATE INDEX idx_shipping_policy_store_id ON shipping_policy(store_id) WHERE deleted_at IS NULL;

-- Delivery slot configuration per D-062
CREATE TABLE delivery_slot_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    allow_consumer_selection BOOLEAN DEFAULT false,
    outer_bound_hours BIGINT, -- Merchant estimate or consumer-selectable window in seconds per D-081
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (outer_bound_hours IS NULL OR outer_bound_hours > 0)
);

COMMENT ON TABLE delivery_slot_config IS 'Controls whether consumers can select delivery date/time windows per D-062';
COMMENT ON COLUMN delivery_slot_config.outer_bound_hours IS 'Stored in seconds per D-081; NULL = no config; consumer selectable if true';

CREATE UNIQUE INDEX idx_delivery_slot_config_store_id ON delivery_slot_config(store_id);

-- Shipping rules per D-062/D-119
CREATE TABLE shipping_rule (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    branch_id UUID REFERENCES branch(id) ON DELETE CASCADE, -- NULL = store-level; branch overrides store per D-119
    governorate TEXT, -- Egyptian governorate, NULL = apply to all
    county TEXT, -- NULL = apply to governorate or all
    district TEXT, -- NULL = apply to county/governorate or all
    estimated_hours BIGINT NOT NULL, -- Delivery time in seconds per D-081
    cost_egp BIGINT NOT NULL, -- In piastres
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (cost_egp >= 0),
    CHECK (estimated_hours > 0)
);

COMMENT ON TABLE shipping_rule IS 'Per-branch or store-level shipping rules per D-119; matched by specificity (district > county > governorate)';
COMMENT ON COLUMN shipping_rule.estimated_hours IS 'Stored in seconds per D-081; merchant estimate or consumer-selectable if configured';

CREATE INDEX idx_shipping_rule_store_id ON shipping_rule(store_id) WHERE is_active = true;
CREATE INDEX idx_shipping_rule_branch_id ON shipping_rule(branch_id) WHERE is_active = true;
CREATE INDEX idx_shipping_rule_location ON shipping_rule(governorate, county, district) WHERE is_active = true;
