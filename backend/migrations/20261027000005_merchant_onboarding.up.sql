-- backend/migrations/20261027000005_merchant_onboarding.up.sql

SET LOCAL search_path = app, public;

-- Merchant applications per D-039
CREATE TABLE merchant_application (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    business_name TEXT NOT NULL,
    contact_name TEXT NOT NULL,
    contact_phone TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    email_verified_at TIMESTAMPTZ,
    product_category TEXT,
    notes TEXT,
    ip_address INET,
    user_agent TEXT,
    state TEXT NOT NULL DEFAULT 'submitted', -- 'submitted', 'contacted', 'approved', 'invited', 'active', 'rejected', 'expired', 'withdrawn'
    rejection_reason TEXT,
    reviewer_id UUID REFERENCES account(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    reviewed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ, -- 90 days from submission
    
    CHECK (state IN ('submitted', 'contacted', 'approved', 'invited', 'active', 'rejected', 'expired', 'withdrawn')),
    CHECK (rejection_reason IS NOT NULL OR state <> 'rejected')
);

COMMENT ON TABLE merchant_application IS 'Merchant application lifecycle per D-039 with durable rejection history for repeat-abuse detection';
COMMENT ON COLUMN merchant_application.state IS 'submitted → contacted → approved → invited → active or rejected/expired/withdrawn';

CREATE INDEX idx_merchant_application_email ON merchant_application(email);
CREATE INDEX idx_merchant_application_state ON merchant_application(state);
CREATE INDEX idx_merchant_application_created_at ON merchant_application(created_at DESC);
CREATE INDEX idx_merchant_application_expires_at ON merchant_application(expires_at);
