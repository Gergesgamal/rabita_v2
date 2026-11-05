-- backend/migrations/20261027000019_feedback.up.sql

SET LOCAL search_path = app, public;

-- Feedback backlog per D-072/D-077
CREATE TABLE feedback_item (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE SET NULL,
    account_type TEXT NOT NULL, -- 'consumer', 'merchant', 'employee', 'admin'
    store_id UUID REFERENCES store(id) ON DELETE SET NULL,
    category TEXT NOT NULL, -- 'feature', 'improvement', 'bug', 'util'
    title TEXT NOT NULL,
    description TEXT,
    state TEXT NOT NULL DEFAULT 'open', -- 'open', 'under_review', 'planned', 'in_progress', 'shipped', 'declined'
    is_public BOOLEAN DEFAULT false, -- Bugs always private; features default private per D-077
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (account_type IN ('consumer', 'merchant', 'employee', 'admin')),
    CHECK (category IN ('feature', 'improvement', 'bug', 'util')),
    CHECK (state IN ('open', 'under_review', 'planned', 'in_progress', 'shipped', 'declined')),
    CHECK ((category = 'bug' AND is_public = false) OR category <> 'bug')
);

COMMENT ON TABLE feedback_item IS 'Feedback backlog per D-072; bugs private, features admin-publishable per D-077';
COMMENT ON COLUMN feedback_item.is_public IS 'Bugs always private; others default private, admin can publish per D-077';

CREATE INDEX idx_feedback_item_account_id ON feedback_item(account_id);
CREATE INDEX idx_feedback_item_store_id ON feedback_item(store_id);
CREATE INDEX idx_feedback_item_state ON feedback_item(state);
CREATE INDEX idx_feedback_item_category ON feedback_item(category);
CREATE INDEX idx_feedback_item_created_at ON feedback_item(created_at DESC);
