-- backend/migrations/20261027000009_catalog.up.sql

SET LOCAL search_path = app, public;

-- Product categories
CREATE TABLE category (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    description TEXT,
    is_published BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    
    UNIQUE (store_id, slug),
    CHECK (slug ~ '^[a-z0-9-]+$')
);

COMMENT ON TABLE category IS 'Store-scoped product categories per D-063';

CREATE INDEX idx_category_store_id ON category(store_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_category_is_published ON category(is_published) WHERE deleted_at IS NULL;

-- Products per D-063
CREATE TABLE product (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    sku TEXT,
    name TEXT NOT NULL,
    description TEXT,
    category_id UUID REFERENCES category(id) ON DELETE SET NULL,
    base_price_egp BIGINT NOT NULL, -- In piastres
    is_published BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    
    UNIQUE (store_id, sku),
    CHECK (base_price_egp >= 0)
);

COMMENT ON TABLE product IS 'Store products with tax-exclusive pricing per D-063/D-118';
COMMENT ON COLUMN product.base_price_egp IS 'In piastres (integers); tax calculated separately at checkout per D-118';

CREATE INDEX idx_product_store_id ON product(store_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_product_is_published ON product(is_published) WHERE deleted_at IS NULL;
CREATE INDEX idx_product_category_id ON product(category_id) WHERE deleted_at IS NULL;

-- Product variants
CREATE TABLE variant (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES product(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    sku TEXT,
    price_override_egp BIGINT, -- If NULL, use product base price
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    
    CHECK (price_override_egp IS NULL OR price_override_egp >= 0)
);

COMMENT ON TABLE variant IS 'Product variants with optional price override per D-063';

CREATE INDEX idx_variant_product_id ON variant(product_id) WHERE deleted_at IS NULL;

-- Product images via media pipeline
CREATE TABLE product_image (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES product(id) ON DELETE CASCADE,
    media_id UUID NOT NULL, -- FK to media table
    display_order SMALLINT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (display_order >= 0)
);

COMMENT ON TABLE product_image IS 'Ordered product images via media pipeline per D-063';

CREATE INDEX idx_product_image_product_id ON product_image(product_id);

-- Per-branch product stock
CREATE TABLE branch_product (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    branch_id UUID NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES product(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES variant(id) ON DELETE CASCADE,
    stock_qty BIGINT NOT NULL DEFAULT 0,
    is_published BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (branch_id, product_id, variant_id),
    CHECK (stock_qty >= 0)
);

COMMENT ON TABLE branch_product IS 'Per-branch inventory per D-062/D-051; stock_qty is canonical source';

CREATE INDEX idx_branch_product_branch_id ON branch_product(branch_id);
CREATE INDEX idx_branch_product_product_id ON branch_product(product_id);
