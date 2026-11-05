-- backend/migrations/20261027000021_consumer_features.up.sql

SET LOCAL search_path = app, public;

-- Consumer address book per D-092
CREATE TABLE address_book (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consumer_id UUID NOT NULL REFERENCES consumer(id) ON DELETE CASCADE,
    label TEXT, -- 'Home', 'Work', etc.
    governorate TEXT,
    county TEXT,
    district TEXT,
    street_address TEXT,
    postal_code TEXT,
    phone_number TEXT,
    gps_latitude DECIMAL(10, 8),
    gps_longitude DECIMAL(11, 8),
    gps_collected_at TIMESTAMPTZ,
    is_default BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (gps_latitude IS NULL OR (gps_latitude >= -90 AND gps_latitude <= 90)),
    CHECK (gps_longitude IS NULL OR (gps_longitude >= -180 AND gps_longitude <= 180))
);

COMMENT ON TABLE address_book IS 'Consumer addresses per D-092; GPS deleted 30 days after delivery or 24h after cancel';

CREATE INDEX idx_address_book_consumer_id ON address_book(consumer_id);

-- Wishlist per D-092
CREATE TABLE wishlist (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consumer_id UUID NOT NULL REFERENCES consumer(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES product(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES variant(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (consumer_id, product_id, variant_id)
);

COMMENT ON TABLE wishlist IS 'Consumer wishlist per D-092';

CREATE INDEX idx_wishlist_consumer_id ON wishlist(consumer_id);

-- Back-in-stock alerts per D-092
CREATE TABLE back_in_stock_alert (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consumer_id UUID NOT NULL REFERENCES consumer(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES product(id) ON DELETE CASCADE,
    branch_id UUID REFERENCES branch(id) ON DELETE CASCADE,
    notified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (consumer_id, product_id, branch_id)
);

COMMENT ON TABLE back_in_stock_alert IS 'Back-in-stock subscriptions per D-092';

CREATE INDEX idx_back_in_stock_alert_consumer_id ON back_in_stock_alert(consumer_id);
CREATE INDEX idx_back_in_stock_alert_product_id ON back_in_stock_alert(product_id);

-- Recently viewed products per D-092
CREATE TABLE recently_viewed (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consumer_id UUID NOT NULL REFERENCES consumer(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES product(id) ON DELETE CASCADE,
    viewed_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (consumer_id, product_id)
);

COMMENT ON TABLE recently_viewed IS 'Recently viewed products per D-092';

CREATE INDEX idx_recently_viewed_consumer_id ON recently_viewed(consumer_id);
CREATE INDEX idx_recently_viewed_viewed_at ON recently_viewed(viewed_at DESC);

-- Consumer consents per D-092
CREATE TABLE consumer_consent (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consumer_id UUID NOT NULL REFERENCES consumer(id) ON DELETE CASCADE,
    consent_type TEXT NOT NULL, -- 'marketing', 'whatsapp', 'analytics', 'third_party'
    granted BOOLEAN DEFAULT false,
    granted_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (consumer_id, consent_type),
    CHECK (consent_type IN ('marketing', 'whatsapp', 'analytics', 'third_party'))
);

COMMENT ON TABLE consumer_consent IS 'Per-consent tracking per D-092';

CREATE INDEX idx_consumer_consent_consumer_id ON consumer_consent(consumer_id);
