-- backend/migrations/20261027000016_refunds.up.sql

SET LOCAL search_path = app, public;

-- Refund requests per D-066 (revised)
CREATE TABLE refund_request (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES "order"(id) ON DELETE RESTRICT,
    consumer_id UUID REFERENCES consumer(id) ON DELETE SET NULL,
    reason TEXT NOT NULL,
    refund_amount_egp BIGINT NOT NULL, -- Full-refund-only per D-121
    state TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'approved', 'rejected', 'escalated_admin', 'issued'
    requested_at TIMESTAMPTZ DEFAULT now(),
    decided_by_id UUID REFERENCES account(id) ON DELETE SET NULL,
    decided_at TIMESTAMPTZ,
    rejection_reason TEXT,
    escalation_reason TEXT,
    admin_override_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (state IN ('pending', 'approved', 'rejected', 'escalated_admin', 'issued')),
    CHECK (refund_amount_egp > 0),
    CHECK (decided_at IS NULL OR state <> 'pending')
);

COMMENT ON TABLE refund_request IS 'Refund requests per D-066/D-121 (full-refund-only); auto-escalates to admin after window';
COMMENT ON COLUMN refund_request.refund_amount_egp IS 'Full-refund-only per D-121: order_subtotal + shipping_cost + tax';

CREATE INDEX idx_refund_request_order_id ON refund_request(order_id);
CREATE INDEX idx_refund_request_consumer_id ON refund_request(consumer_id);
CREATE INDEX idx_refund_request_state ON refund_request(state);
CREATE INDEX idx_refund_request_requested_at ON refund_request(requested_at DESC);

-- Refund decision audit
CREATE TABLE refund_decision (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    refund_request_id UUID NOT NULL REFERENCES refund_request(id) ON DELETE CASCADE,
    decision TEXT NOT NULL, -- 'approved', 'rejected'
    decided_by_id UUID NOT NULL REFERENCES account(id) ON DELETE SET NULL,
    reason TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (decision IN ('approved', 'rejected'))
);

COMMENT ON TABLE refund_decision IS 'Audit trail of refund approval/rejection';

CREATE INDEX idx_refund_decision_refund_request_id ON refund_decision(refund_request_id);
