-- backend/migrations/20261027000008_branches.up.sql

SET LOCAL search_path = app, public;

-- Branch per D-062
CREATE TABLE branch (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    address TEXT,
    phone_number TEXT,
    is_published BOOLEAN DEFAULT false,
    pickup_enabled BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ
);

COMMENT ON TABLE branch IS 'Branch locations per D-062, each with independent stock and shipping rules';

CREATE INDEX idx_branch_store_id ON branch(store_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_branch_is_published ON branch(is_published) WHERE deleted_at IS NULL;

-- Branch working hours per D-062
CREATE TABLE branch_working_hours (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    branch_id UUID NOT NULL REFERENCES branch(id) ON DELETE CASCADE,
    day_of_week SMALLINT NOT NULL, -- 0=Sunday, 6=Saturday
    opens_at TIME NOT NULL,
    closes_at TIME NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (day_of_week >= 0 AND day_of_week <= 6),
    UNIQUE (branch_id, day_of_week)
);

COMMENT ON TABLE branch_working_hours IS 'Operating hours per branch per day, supporting partial days per D-062';

CREATE INDEX idx_branch_working_hours_branch_id ON branch_working_hours(branch_id);
