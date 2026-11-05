-- backend/migrations/20261027000012_orders.up.sql

SET LOCAL search_path = app, public;

-- Order statuses: platform categories with optional merchant customization
CREATE TABLE order_status_default (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    platform_category TEXT UNIQUE NOT NULL, -- 'pending', 'confirmed', 'preparing', 'shipped', 'delivered', 'cancelled', 'returned', 'failed', 'on_hold'
    label TEXT NOT NULL,
    description TEXT,
    financial_category TEXT NOT NULL, -- 'active', 'delivered', 'cancelled', 'returned'
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (platform_category IN ('pending', 'confirmed', 'preparing', 'shipped', 'delivered', 'cancelled', 'returned', 'failed', 'on_hold')),
    CHECK (financial_category IN ('active', 'delivered', 'cancelled', 'returned'))
);

COMMENT ON TABLE order_status_default IS 'Platform order status categories per D-050; merchant customization overrides these';

-- Merchant customizations of order status labels
CREATE TABLE order_status_customization (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    platform_category TEXT NOT NULL REFERENCES order_status_default(platform_category) ON DELETE CASCADE,
    label TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (store_id, platform_category)
);

COMMENT ON TABLE order_status_customization IS 'Merchant-customized status labels per D-050; resets to default if deleted';

CREATE INDEX idx_order_status_customization_store_id ON order_status_customization(store_id);

-- Payment status defaults
CREATE TABLE payment_status_default (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    platform_category TEXT UNIQUE NOT NULL, -- 'delayed', 'paid', 'cod', 'refunded'
    label TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (platform_category IN ('delayed', 'paid', 'cod', 'refunded'))
);

COMMENT ON TABLE payment_status_default IS 'Payment status categories per D-050';

-- Merchant customizations of payment status
CREATE TABLE payment_status_customization (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    platform_category TEXT NOT NULL REFERENCES payment_status_default(platform_category) ON DELETE CASCADE,
    label TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (store_id, platform_category)
);

COMMENT ON TABLE payment_status_customization IS 'Merchant-customized payment status labels';

CREATE INDEX idx_payment_status_customization_store_id ON payment_status_customization(store_id);

-- Orders per D-051/D-114
CREATE TABLE "order" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE RESTRICT,
    public_order_number TEXT UNIQUE NOT NULL, -- e.g., 'rabita-7XYZ3A' per D-120
    consumer_id UUID REFERENCES consumer(id) ON DELETE SET NULL,
    guest_email TEXT, -- For guest orders
    guest_phone TEXT,
    guest_session_id UUID, -- For guest order claim per D-093
    order_platform_category TEXT NOT NULL REFERENCES order_status_default(platform_category),
    payment_platform_category TEXT NOT NULL DEFAULT 'delayed' REFERENCES payment_status_default(platform_category),
    verification_method TEXT, -- 'email_only', 'whatsapp_otp', 'phone_not_on_whatsapp' per D-128
    order_snapshot JSONB NOT NULL, -- Immutable snapshot per D-114
    shipping_address_snapshot JSONB NOT NULL, -- Complete address per D-062
    shipping_cost_egp BIGINT NOT NULL, -- In piastres
    shipping_method TEXT, -- Merchant-customizable label
    estimated_delivery_hours BIGINT, -- In seconds per D-081
    consumer_note TEXT,
    tracking_number TEXT,
    carrier TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    confirmed_at TIMESTAMPTZ, -- When merchant confirms (stock decremented)
    
    CHECK (shipping_cost_egp >= 0),
    CHECK (estimated_delivery_hours IS NULL OR estimated_delivery_hours > 0),
    CHECK (order_platform_category IN ('pending', 'confirmed', 'preparing', 'shipped', 'delivered', 'cancelled', 'returned', 'failed', 'on_hold')),
    CHECK (payment_platform_category IN ('delayed', 'paid', 'cod', 'refunded')),
    CHECK (consumer_id IS NOT NULL OR guest_email IS NOT NULL)
);

COMMENT ON TABLE "order" IS 'Orders with immutable snapshot per D-114; stock reserved at creation, decremented at confirmation per D-051';
COMMENT ON COLUMN "order".public_order_number IS 'Base32 public-facing order number per D-120, never internal ID';
COMMENT ON COLUMN "order".order_snapshot IS 'Immutable JSONB: {product_title, variant_title, sku, unit_price, quantity, tax_rate, tax_amount, subtotal, product_image_s3_key}';

CREATE INDEX idx_order_store_id ON "order"(store_id);
CREATE INDEX idx_order_consumer_id ON "order"(consumer_id) WHERE consumer_id IS NOT NULL;
CREATE INDEX idx_order_guest_email ON "order"(guest_email) WHERE guest_email IS NOT NULL;
CREATE INDEX idx_order_platform_category ON "order"(order_platform_category);
CREATE INDEX idx_order_created_at ON "order"(created_at DESC);
CREATE INDEX idx_order_public_order_number ON "order"(public_order_number);

-- Order items per D-114 (mostly in snapshot, but queryable rows for reports)
CREATE TABLE order_item (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES "order"(id) ON DELETE RESTRICT,
    product_id UUID NOT NULL REFERENCES product(id) ON DELETE RESTRICT,
    variant_id UUID REFERENCES variant(id) ON DELETE SET NULL,
    branch_id UUID NOT NULL REFERENCES branch(id) ON DELETE RESTRICT,
    quantity SMALLINT NOT NULL,
    unit_price_egp BIGINT NOT NULL, -- In piastres
    line_tax_rate_bp SMALLINT NOT NULL, -- Basis points per D-118
    line_tax_amount_egp BIGINT NOT NULL,
    line_subtotal_egp BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (quantity > 0),
    CHECK (unit_price_egp >= 0),
    CHECK (line_tax_rate_bp >= 0),
    CHECK (line_tax_amount_egp >= 0),
    CHECK (line_subtotal_egp >= 0)
);

COMMENT ON TABLE order_item IS 'Order line items per D-114; data also frozen in order_snapshot JSONB';

CREATE INDEX idx_order_item_order_id ON order_item(order_id);
CREATE INDEX idx_order_item_product_id ON order_item(product_id);

-- Stock reserve records per D-051 (revised)
CREATE TABLE stock_reserve (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES "order"(id) ON DELETE RESTRICT,
    product_id UUID NOT NULL REFERENCES product(id) ON DELETE RESTRICT,
    variant_id UUID REFERENCES variant(id) ON DELETE SET NULL,
    branch_id UUID NOT NULL REFERENCES branch(id) ON DELETE RESTRICT,
    quantity SMALLINT NOT NULL,
    state TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'active', 'released', 'shadowed'
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (quantity > 0),
    CHECK (state IN ('pending', 'active', 'released', 'shadowed'))
);

COMMENT ON TABLE stock_reserve IS 'Stock reservation per D-051 (revised); pending at creation, active at confirmation, released on cancel/return, shadowed on permitted deletion';

CREATE INDEX idx_stock_reserve_order_id ON stock_reserve(order_id);
CREATE INDEX idx_stock_reserve_branch_product ON stock_reserve(branch_id, product_id);
CREATE INDEX idx_stock_reserve_state ON stock_reserve(state);

-- Order status history for audit trail
CREATE TABLE order_status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES "order"(id) ON DELETE CASCADE,
    platform_category TEXT NOT NULL REFERENCES order_status_default(platform_category),
    actor_id UUID REFERENCES account(id) ON DELETE SET NULL,
    reason TEXT,
    created_at TIMESTAMPTZ DEFAULT now()
);

COMMENT ON TABLE order_status_history IS 'Immutable audit of status transitions per D-050';

CREATE INDEX idx_order_status_history_order_id ON order_status_history(order_id);
CREATE INDEX idx_order_status_history_created_at ON order_status_history(created_at DESC);

-- Seed default statuses
INSERT INTO order_status_default (platform_category, label, description, financial_category) VALUES
('pending', 'Pending', 'Awaiting confirmation', 'active'),
('confirmed', 'Confirmed', 'Merchant confirmed order', 'active'),
('preparing', 'Preparing', 'Items being prepared', 'active'),
('shipped', 'Shipped', 'Order dispatched', 'active'),
('delivered', 'Delivered', 'Order received', 'delivered'),
('cancelled', 'Cancelled', 'Order cancelled', 'cancelled'),
('returned', 'Returned', 'Order returned', 'returned'),
('failed', 'Failed', 'Fulfillment failed', 'cancelled'),
('on_hold', 'On Hold', 'Order on hold pending action', 'active');

INSERT INTO payment_status_default (platform_category, label) VALUES
('delayed', 'Payment Pending'),
('paid', 'Payment Received'),
('cod', 'Cash on Delivery'),
('refunded', 'Refunded');
