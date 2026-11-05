-- backend/migrations/20261027000015_cleanup.up.sql

SET LOCAL search_path = app, public;

-- Cleanup rules per order category per D-052
CREATE TABLE cleanup_rule (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    order_platform_category TEXT NOT NULL REFERENCES order_status_default(platform_category),
    days_to_cleanup SMALLINT NOT NULL,
    days_to_retain_audit SMALLINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (store_id, order_platform_category),
    CHECK (days_to_cleanup > 0),
    CHECK (days_to_retain_audit >= days_to_cleanup)
);

COMMENT ON TABLE cleanup_rule IS 'Per-store cleanup rules per order category per D-052; evaluated by scheduled worker';

CREATE INDEX idx_cleanup_rule_store_id ON cleanup_rule(store_id);

-- Cleanup run audit
CREATE TABLE cleanup_run (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    rule_id UUID NOT NULL REFERENCES cleanup_rule(id) ON DELETE SET NULL,
    dry_run BOOLEAN DEFAULT true,
    affected_order_count BIGINT NOT NULL,
    deleted_order_count BIGINT NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'running', 'completed', 'failed'
    error_message TEXT,
    manifest JSONB, -- Detailed deletion manifest
    created_at TIMESTAMPTZ DEFAULT now(),
    completed_at TIMESTAMPTZ,
    
    CHECK (status IN ('pending', 'running', 'completed', 'failed')),
    CHECK (deleted_order_count >= 0),
    CHECK (affected_order_count >= 0)
);

COMMENT ON TABLE cleanup_run IS 'Audit trail for cleanup execution per D-052, with idempotent manifests';

CREATE INDEX idx_cleanup_run_store_id ON cleanup_run(store_id);
CREATE INDEX idx_cleanup_run_status ON cleanup_run(status);
CREATE INDEX idx_cleanup_run_created_at ON cleanup_run(created_at DESC);
