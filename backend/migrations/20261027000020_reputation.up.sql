-- backend/migrations/20261027000020_reputation.up.sql

SET LOCAL search_path = app, public;

-- Private reputation/risk signal per D-079 (owner-only visibility)
CREATE TABLE reputation (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL UNIQUE REFERENCES account(id) ON DELETE CASCADE,
    signed_score BIGINT DEFAULT 0, -- Owner-only visibility; no automation until approved per D-079
    last_updated_at TIMESTAMPTZ DEFAULT now(),
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (signed_score >= -1000000 AND signed_score <= 1000000)
);

COMMENT ON TABLE reputation IS 'Private owner-only reputation score per D-079; no automated actions until approved';
COMMENT ON COLUMN reputation.signed_score IS 'Visible only to owner; does not drive automated actions in MVP';

CREATE INDEX idx_reputation_account_id ON reputation(account_id);

-- Reputation event audit (append-only)
CREATE TABLE reputation_event (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reputation_id UUID NOT NULL REFERENCES reputation(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL, -- 'refund_declined', 'abuse_report', 'support_escalation', etc.
    weight BIGINT,
    reason TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (weight >= -1000 AND weight <= 1000)
);

COMMENT ON TABLE reputation_event IS 'Append-only events per D-079; weights not yet specified, no automation';

CREATE INDEX idx_reputation_event_reputation_id ON reputation_event(reputation_id);
CREATE INDEX idx_reputation_event_created_at ON reputation_event(created_at DESC);
