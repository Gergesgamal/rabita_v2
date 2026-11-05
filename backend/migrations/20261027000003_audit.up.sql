-- backend/migrations/20261027000003_audit.up.sql

SET LOCAL search_path = app, public;

-- Audit log with hash-chain for critical events
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type TEXT NOT NULL, -- 'permission_change', 'account_delete', 'order_create', etc.
    actor_id UUID, -- REFERENCES account(id), may be NULL for system events
    actor_type TEXT, -- 'account', 'system', 'service'
    target_id UUID,
    target_type TEXT, -- 'account', 'store', 'order', 'merchant_application', etc.
    reason TEXT,
    payload JSONB,
    previous_state JSONB,
    new_state JSONB,
    payload_hash TEXT, -- SHA256 hex of payload, for integrity
    previous_hash TEXT, -- Hash of previous audit record (hash-chain)
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (event_type ~ '^[a-z_]+$')
);

COMMENT ON TABLE audit_log IS 'Append-only audit with hash-chain per D-023; never contains passwords, OTPs, tokens, secrets';
COMMENT ON COLUMN audit_log.payload_hash IS 'SHA256 hash of payload for integrity verification';
COMMENT ON COLUMN audit_log.previous_hash IS 'Hash of previous record for tamper detection';

CREATE INDEX idx_audit_log_event_type ON audit_log(event_type);
CREATE INDEX idx_audit_log_actor_id ON audit_log(actor_id);
CREATE INDEX idx_audit_log_target_id ON audit_log(target_id);
CREATE INDEX idx_audit_log_created_at ON audit_log(created_at DESC);
CREATE INDEX idx_audit_log_target_type_id ON audit_log(target_type, target_id);

-- Ensure audit table is append-only in practice
-- (In production, add triggers to prevent UPDATE/DELETE)
