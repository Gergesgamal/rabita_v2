-- backend/migrations/20261027000023_components.up.sql

SET LOCAL search_path = app, public;

-- Custom components per D-032
CREATE TABLE puck_component (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID REFERENCES store(id) ON DELETE CASCADE,
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    component_name TEXT NOT NULL,
    component_code TEXT NOT NULL, -- JSX/TSX source
    compiled_code TEXT, -- Compiled artifact from sucrase/esbuild
    approval_state TEXT DEFAULT 'pending_approval', -- 'pending_approval', 'active', 'rejected'
    approval_reason TEXT,
    rejected_reason TEXT,
    approved_by_id UUID REFERENCES account(id) ON DELETE SET NULL,
    is_self_authored BOOLEAN DEFAULT false, -- True if super admin authored per D-032
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deprecated_at TIMESTAMPTZ,
    
    CHECK (approval_state IN ('pending_approval', 'active', 'rejected')),
    CHECK (component_name ~ '^[A-Z][a-zA-Z0-9]*$')
);

COMMENT ON TABLE puck_component IS 'UI-authored components per D-032; Tier B sandbox, approval gate ON by default';
COMMENT ON COLUMN puck_component.approval_state IS 'pending_approval → active or rejected; self-authored super admin components auto-approved but marked';
COMMENT ON COLUMN puck_component.compiled_code IS 'Runtime-compiled from source using sucrase or esbuild-wasm per D-032';

CREATE INDEX idx_puck_component_account_id ON puck_component(account_id);
CREATE INDEX idx_puck_component_store_id ON puck_component(store_id);
CREATE INDEX idx_puck_component_approval_state ON puck_component(approval_state);

-- Component approval
CREATE TABLE puck_component_approval (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    component_id UUID NOT NULL REFERENCES puck_component(id) ON DELETE CASCADE,
    approver_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    decision TEXT NOT NULL, -- 'approved', 'rejected'
    reason TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (decision IN ('approved', 'rejected'))
);

COMMENT ON TABLE puck_component_approval IS 'Approval audit per D-032';

CREATE INDEX idx_puck_component_approval_component_id ON puck_component_approval(component_id);
