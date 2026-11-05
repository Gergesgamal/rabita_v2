-- backend/migrations/20261027000006_delegation.up.sql

SET LOCAL search_path = app, public;

-- Permission catalog (seeded with all 30 capabilities)
CREATE TABLE permission_catalog (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    capability_name TEXT UNIQUE NOT NULL, -- e.g., 'catalog.read', 'orders.refund', 'permissions.grant'
    description TEXT,
    is_delegable BOOLEAN DEFAULT true,
    is_dual_control BOOLEAN DEFAULT false, -- Requires approval workflow
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (capability_name ~ '^[a-z]+(\.[a-z_]+)+$')
);

COMMENT ON TABLE permission_catalog IS 'Complete permission catalog with 30+ capabilities per D-009';
COMMENT ON COLUMN permission_catalog.is_delegable IS 'If false, cannot be granted to others (e.g., permissions.grant cannot be sub-delegated)';
COMMENT ON COLUMN permission_catalog.is_dual_control IS 'If true, requires approval workflow per D-026';

-- Account permissions (many-to-many with context)
CREATE TABLE account_permission (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    context_id UUID NOT NULL, -- store_id or admin context
    capability_id UUID NOT NULL REFERENCES permission_catalog(id) ON DELETE CASCADE,
    granted_by_id UUID REFERENCES account(id) ON DELETE SET NULL,
    granted_at TIMESTAMPTZ DEFAULT now(),
    expires_at TIMESTAMPTZ, -- For time-bounded grants (dual-control)
    version BIGINT DEFAULT 1, -- For permission-version-based revocation
    created_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (account_id, context_id, capability_id),
    CHECK (expires_at IS NULL OR expires_at > granted_at)
);

COMMENT ON TABLE account_permission IS 'Capability grants per context with optional expiry per D-026';
COMMENT ON COLUMN account_permission.version IS 'Incremented on permission changes; sessions use this for revocation detection';

CREATE INDEX idx_account_permission_account_id ON account_permission(account_id);
CREATE INDEX idx_account_permission_context_id ON account_permission(context_id);
CREATE INDEX idx_account_permission_expires_at ON account_permission(expires_at);

-- Approval requests for dual-control actions per D-026
CREATE TABLE approval_request (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    context_id UUID NOT NULL,
    action_type TEXT NOT NULL, -- 'employees.manage', 'orders.refund', etc.
    action_payload JSONB NOT NULL,
    payload_hash TEXT NOT NULL, -- SHA256
    state TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'approved', 'rejected'
    approved_by_id UUID REFERENCES account(id) ON DELETE SET NULL,
    approval_reason TEXT,
    rejected_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    decided_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ NOT NULL,
    
    CHECK (state IN ('pending', 'approved', 'rejected')),
    CHECK (decided_at IS NULL OR state <> 'pending'),
    CHECK (expires_at > created_at)
);

COMMENT ON TABLE approval_request IS 'Dual-control approval workflow per D-026, bound to exact payload hash';
COMMENT ON COLUMN approval_request.payload_hash IS 'SHA256 of action_payload; mutated payloads create 409 Conflict on execution';

CREATE INDEX idx_approval_request_account_id ON approval_request(account_id) WHERE state = 'pending';
CREATE INDEX idx_approval_request_context_id ON approval_request(context_id) WHERE state = 'pending';
CREATE INDEX idx_approval_request_state ON approval_request(state);
CREATE INDEX idx_approval_request_expires_at ON approval_request(expires_at);

-- Execution tokens bound to approval request per D-026
CREATE TABLE execution_token (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    approval_request_id UUID NOT NULL REFERENCES approval_request(id) ON DELETE CASCADE,
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    employee_account_id UUID REFERENCES account(id) ON DELETE CASCADE,
    token_hash TEXT UNIQUE NOT NULL,
    payload_hash TEXT NOT NULL, -- Must match approval request
    executed BOOLEAN DEFAULT false,
    executed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    
    CHECK (expires_at > created_at)
);

COMMENT ON TABLE execution_token IS 'Single-use token bound to approval request and payload hash per D-026';
COMMENT ON COLUMN execution_token.payload_hash IS 'Mutated payload at execution returns 409 Conflict';

CREATE INDEX idx_execution_token_token_hash ON execution_token(token_hash) WHERE executed = false;
CREATE INDEX idx_execution_token_expires_at ON execution_token(expires_at);

-- Seed permission catalog with 30+ capabilities
INSERT INTO permission_catalog (capability_name, description, is_delegable, is_dual_control) VALUES
('catalog.read', 'Read product catalog', true, false),
('catalog.write', 'Create/edit products', true, false),
('catalog.delete', 'Delete products', true, false),
('inventory.adjust', 'Adjust branch stock', true, false),
('orders.read', 'View orders', true, false),
('orders.fulfill', 'Mark orders as shipped/delivered', true, false),
('orders.cancel', 'Cancel orders', true, false),
('orders.refund', 'Approve/process refunds', false, true),
('customer.read', 'View customer data', true, false),
('customer.message', 'Send transactional messages to customers', true, false),
('support.read', 'Read support tickets', true, false),
('support.respond', 'Reply to tickets', true, false),
('support.escalate', 'Escalate tickets to admin', true, false),
('content.edit', 'Edit customizable pages', true, false),
('content.publish', 'Publish pages', true, false),
('analytics.view', 'View analytics dashboard', true, false),
('promotions.manage', 'Create/edit discounts and coupons', true, false),
('jobs.manage', 'Post and manage jobs', true, false),
('jobs.review', 'Review job applications', true, false),
('employees.manage', 'Hire/fire employees, assign permissions', false, true),
('permissions.grant', 'Grant/revoke capabilities', false, true),
('store.delete', 'Delete a store', false, true),
('store.settings.billing', 'Manage store billing and payment settings', false, true),
('store.transfer', 'Transfer store ownership', false, true),
('apikeys.manage', 'Manage API keys and webhooks', false, true),
('webhooks.manage', 'Configure and test webhooks', false, true),
('shipping.configure', 'Set shipping rules and policies', true, false),
('media.upload', 'Upload media to store', true, false),
('components.author', 'Create custom components', true, false),
('components.approve', 'Approve custom components', false, false);
