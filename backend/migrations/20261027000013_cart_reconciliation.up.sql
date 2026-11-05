-- backend/migrations/20261027000013_cart_reconciliation.up.sql

SET LOCAL search_path = app, public;

-- Cart audit for reconciliation between Valkey and DB per D-115
CREATE TABLE cart_audit (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    consumer_id UUID REFERENCES consumer(id) ON DELETE SET NULL,
    guest_session_id UUID,
    action TEXT NOT NULL, -- 'created', 'updated', 'merged', 'cleared', 'abandoned'
    line_count SMALLINT,
    total_egp BIGINT, -- In piastres
    payload JSONB,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (action IN ('created', 'updated', 'merged', 'cleared', 'abandoned'))
);

COMMENT ON TABLE cart_audit IS 'Audit trail for cart operations; Valkey is ephemeral, this records events per D-115';

CREATE INDEX idx_cart_audit_consumer_id ON cart_audit(consumer_id);
CREATE INDEX idx_cart_audit_guest_session_id ON cart_audit(guest_session_id);
CREATE INDEX idx_cart_audit_store_id ON cart_audit(store_id);
CREATE INDEX idx_cart_audit_created_at ON cart_audit(created_at DESC);
