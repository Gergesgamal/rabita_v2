-- backend/migrations/20261027000017_support.up.sql

SET LOCAL search_path = app, public;

-- Support tickets per D-073 (revised)
CREATE TABLE ticket (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    consumer_id UUID REFERENCES consumer(id) ON DELETE SET NULL,
    order_id UUID REFERENCES "order"(id) ON DELETE SET NULL,
    refund_request_id UUID REFERENCES refund_request(id) ON DELETE SET NULL,
    subject TEXT NOT NULL,
    description TEXT,
    state TEXT NOT NULL DEFAULT 'pending_support', -- 'pending_support', 'pending_user', 'resolved', 'closed', 'reopened'
    priority SMALLINT DEFAULT 1,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    resolved_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    auto_closed_at TIMESTAMPTZ,
    reopen_deadline TIMESTAMPTZ, -- 7 days after close per D-073
    
    CHECK (state IN ('pending_support', 'pending_user', 'resolved', 'closed', 'reopened')),
    CHECK (priority >= 0 AND priority <= 5),
    CHECK ((order_id IS NOT NULL OR refund_request_id IS NOT NULL) OR store_id IS NOT NULL)
);

COMMENT ON TABLE ticket IS 'Support tickets per D-073; escalates at earlier of 7 days OR 50% of lifetime per D-094';
COMMENT ON COLUMN ticket.state IS 'pending_support ↔ pending_user ↔ resolved → closed; reopenable within 7 days';

CREATE INDEX idx_ticket_store_id ON ticket(store_id) WHERE state IN ('pending_support', 'pending_user', 'reopened');
CREATE INDEX idx_ticket_consumer_id ON ticket(consumer_id);
CREATE INDEX idx_ticket_order_id ON ticket(order_id);
CREATE INDEX idx_ticket_state ON ticket(state);
CREATE INDEX idx_ticket_created_at ON ticket(created_at DESC);
CREATE INDEX idx_ticket_reopen_deadline ON ticket(reopen_deadline) WHERE state = 'closed';

-- Ticket replies
CREATE TABLE ticket_reply (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES ticket(id) ON DELETE CASCADE,
    account_id UUID REFERENCES account(id) ON DELETE SET NULL, -- Support agent or merchant
    consumer_id UUID REFERENCES consumer(id) ON DELETE SET NULL, -- Consumer reply
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK ((account_id IS NOT NULL AND consumer_id IS NULL) OR (account_id IS NULL AND consumer_id IS NOT NULL))
);

COMMENT ON TABLE ticket_reply IS 'Replies to tickets; append-only, cannot be edited/deleted';

CREATE INDEX idx_ticket_reply_ticket_id ON ticket_reply(ticket_id);
CREATE INDEX idx_ticket_reply_created_at ON ticket_reply(created_at DESC);

-- Ticket attachments via media pipeline
CREATE TABLE ticket_attachment (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES ticket(id) ON DELETE CASCADE,
    media_id UUID NOT NULL, -- REFERENCES media table
    created_at TIMESTAMPTZ DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL, -- 3 days after ticket close per D-073
    
    CHECK (expires_at > created_at)
);

COMMENT ON TABLE ticket_attachment IS 'Scanned attachments per D-073; auto-deleted 3 days after ticket close';

CREATE INDEX idx_ticket_attachment_ticket_id ON ticket_attachment(ticket_id);
CREATE INDEX idx_ticket_attachment_expires_at ON ticket_attachment(expires_at);

-- Per-actor last-viewed timestamp for blue-dot indicators
CREATE TABLE ticket_last_viewed (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID NOT NULL REFERENCES ticket(id) ON DELETE CASCADE,
    account_id UUID REFERENCES account(id) ON DELETE CASCADE,
    consumer_id UUID REFERENCES consumer(id) ON DELETE CASCADE,
    last_viewed_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (ticket_id, account_id),
    CHECK ((account_id IS NOT NULL AND consumer_id IS NULL) OR (account_id IS NULL AND consumer_id IS NOT NULL))
);

COMMENT ON TABLE ticket_last_viewed IS 'Per-actor last-viewed timestamp for unread indicators per D-073';

CREATE INDEX idx_ticket_last_viewed_ticket_id ON ticket_last_viewed(ticket_id);
