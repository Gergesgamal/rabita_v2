-- backend/migrations/20261027000014_promotions.up.sql

SET LOCAL search_path = app, public;

-- Discounts (per-product) per D-053
CREATE TABLE discount (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES product(id) ON DELETE CASCADE,
    discount_type TEXT NOT NULL, -- 'absolute_egp', 'percentage'
    discount_value BIGINT NOT NULL, -- Piastres or basis points
    is_active BOOLEAN DEFAULT true,
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (discount_type IN ('absolute_egp', 'percentage')),
    CHECK (discount_value > 0),
    CHECK (end_date IS NULL OR end_date > start_date)
);

COMMENT ON TABLE discount IS 'Per-product discounts per D-053';

CREATE INDEX idx_discount_store_id ON discount(store_id) WHERE is_active = true;
CREATE INDEX idx_discount_product_id ON discount(product_id) WHERE is_active = true;

-- Coupons (store or branch level) per D-053
CREATE TABLE coupon (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    branch_id UUID REFERENCES branch(id) ON DELETE CASCADE, -- NULL = store-level
    code TEXT NOT NULL,
    discount_type TEXT NOT NULL, -- 'absolute_egp', 'percentage'
    discount_value BIGINT NOT NULL,
    usage_limit BIGINT, -- NULL = unlimited
    usage_count BIGINT NOT NULL DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (store_id, code),
    CHECK (discount_type IN ('absolute_egp', 'percentage')),
    CHECK (discount_value > 0),
    CHECK (usage_limit IS NULL OR usage_limit > 0),
    CHECK (usage_count >= 0),
    CHECK (end_date IS NULL OR end_date > start_date)
);

COMMENT ON TABLE coupon IS 'Store or branch-level coupons per D-053; branch overrides store';

CREATE INDEX idx_coupon_store_id ON coupon(store_id) WHERE is_active = true;
CREATE INDEX idx_coupon_code ON coupon(code) WHERE is_active = true;

-- Bundles with partial removal per D-053
CREATE TABLE bundle (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    discount_type TEXT NOT NULL, -- 'absolute_egp', 'percentage'
    discount_value BIGINT NOT NULL,
    allow_partial_removal BOOLEAN DEFAULT false,
    max_partial_applications_per_order SMALLINT DEFAULT 1,
    is_active BOOLEAN DEFAULT true,
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (discount_type IN ('absolute_egp', 'percentage')),
    CHECK (discount_value > 0),
    CHECK (max_partial_applications_per_order > 0),
    CHECK (end_date IS NULL OR end_date > start_date)
);

COMMENT ON TABLE bundle IS 'Product bundles with optional partial removal per D-053';

CREATE INDEX idx_bundle_store_id ON bundle(store_id) WHERE is_active = true;

-- Bundle items
CREATE TABLE bundle_item (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bundle_id UUID NOT NULL REFERENCES bundle(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES product(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES variant(id) ON DELETE SET NULL,
    quantity SMALLINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (quantity > 0)
);

COMMENT ON TABLE bundle_item IS 'Products included in bundle';

CREATE INDEX idx_bundle_item_bundle_id ON bundle_item(bundle_id);

-- Bundle partial application audit per D-053
CREATE TABLE bundle_partial_application (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bundle_id UUID NOT NULL REFERENCES bundle(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES "order"(id) ON DELETE CASCADE,
    actor_id UUID REFERENCES account(id) ON DELETE SET NULL,
    original_items JSONB NOT NULL, -- JSON array of original bundle items
    removed_items JSONB NOT NULL, -- JSON array of removed items
    resulting_price_egp BIGINT NOT NULL, -- Recomputed price in piastres
    discount_applied_egp BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (resulting_price_egp >= 0),
    CHECK (discount_applied_egp >= 0)
);

COMMENT ON TABLE bundle_partial_application IS 'Audit of partial bundle removals with recomputed pricing per D-053';

CREATE INDEX idx_bundle_partial_application_bundle_id ON bundle_partial_application(bundle_id);
CREATE INDEX idx_bundle_partial_application_order_id ON bundle_partial_application(order_id);
