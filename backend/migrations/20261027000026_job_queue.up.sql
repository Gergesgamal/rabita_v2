-- backend/migrations/20261027000026_job_queue.up.sql

SET LOCAL search_path = app, public;

-- Outbox pattern for reliable async delivery per D-084
CREATE TABLE outbox (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type TEXT NOT NULL, -- 'email.send', 'notification.create', 'cleanup.run', etc.
    principal_type TEXT NOT NULL, -- 'account', 'consumer', 'service'
    principal_id TEXT NOT NULL, -- For idempotency per D-130
    payload JSONB NOT NULL,
    idempotency_key TEXT, -- For deduplication
    state TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed', 'dlq'
    retry_count BIGINT DEFAULT 0,
    max_retries BIGINT DEFAULT 3,
    last_error TEXT,
    lease_until TIMESTAMPTZ, -- For crash recovery
    created_at TIMESTAMPTZ DEFAULT now(),
    completed_at TIMESTAMPTZ,
    
    CHECK (state IN ('pending', 'processing', 'completed', 'failed', 'dlq')),
    CHECK (retry_count >= 0)
);

COMMENT ON TABLE outbox IS 'Outbox pattern per D-084 with lease expiry for crash recovery';
COMMENT ON COLUMN outbox.lease_until IS 'Worker heartbeat; if expired, lease taken by another worker';

CREATE INDEX idx_outbox_state ON outbox(state);
CREATE INDEX idx_outbox_event_type ON outbox(event_type) WHERE state = 'pending';
CREATE INDEX idx_outbox_lease_until ON outbox(lease_until) WHERE state = 'processing';
CREATE INDEX idx_outbox_created_at ON outbox(created_at DESC);

-- Outbox attempt audit
CREATE TABLE outbox_attempt (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    outbox_id UUID NOT NULL REFERENCES outbox(id) ON DELETE CASCADE,
    attempt_number BIGINT NOT NULL,
    result TEXT, -- 'success', 'temporary_failure', 'permanent_failure'
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (attempt_number > 0),
    CHECK (result IN ('success', 'temporary_failure', 'permanent_failure'))
);

COMMENT ON TABLE outbox_attempt IS 'Detailed attempt history per D-084';

CREATE INDEX idx_outbox_attempt_outbox_id ON outbox_attempt(outbox_id);
