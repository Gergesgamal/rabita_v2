-- backend/migrations/20261027000018_jobs.up.sql

SET LOCAL search_path = app, public;

-- Job posts per D-074
CREATE TABLE job_post (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    state TEXT NOT NULL DEFAULT 'draft', -- 'draft', 'public', 'unpublished', 'archived'
    is_indexable BOOLEAN DEFAULT true, -- Robots.txt, sitemap per D-074
    contact_phone TEXT NOT NULL, -- Merchant's own number per D-074
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    published_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    
    CHECK (state IN ('draft', 'public', 'unpublished', 'archived')),
    CHECK (contact_phone ~ '^[0-9+\-() ]+$')
);

COMMENT ON TABLE job_post IS 'Job listings per D-074; public posts are indexable with schema.org/JobPosting';

CREATE INDEX idx_job_post_store_id ON job_post(store_id) WHERE state = 'public';
CREATE INDEX idx_job_post_state ON job_post(state);
CREATE INDEX idx_job_post_created_at ON job_post(created_at DESC);

-- Job applications
CREATE TABLE job_application (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_post_id UUID NOT NULL REFERENCES job_post(id) ON DELETE CASCADE,
    applicant_email TEXT NOT NULL,
    applicant_phone TEXT,
    applicant_name TEXT NOT NULL,
    cv_media_id UUID, -- REFERENCES media table; scanned via pipeline
    cover_letter TEXT,
    state TEXT NOT NULL DEFAULT 'applied', -- 'applied', 'reviewed', 'accepted', 'rejected', 'withdrawn'
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    decided_at TIMESTAMPTZ,
    
    CHECK (state IN ('applied', 'reviewed', 'accepted', 'rejected', 'withdrawn'))
);

COMMENT ON TABLE job_application IS 'Applications per D-074; accepted applications create EmployeeRelation';

CREATE INDEX idx_job_application_job_post_id ON job_application(job_post_id) WHERE state IN ('applied', 'reviewed');
CREATE INDEX idx_job_application_state ON job_application(state);
CREATE INDEX idx_job_application_created_at ON job_application(created_at DESC);

-- Job application chat per D-030
CREATE TABLE job_chat_thread (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_application_id UUID NOT NULL UNIQUE REFERENCES job_application(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

COMMENT ON TABLE job_chat_thread IS 'Chat thread per application; polling transport per D-030';

-- Chat messages
CREATE TABLE job_chat_message (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    thread_id UUID NOT NULL REFERENCES job_chat_thread(id) ON DELETE CASCADE,
    account_id UUID REFERENCES account(id) ON DELETE SET NULL, -- Merchant
    applicant_id UUID REFERENCES job_application(id) ON DELETE SET NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK ((account_id IS NOT NULL AND applicant_id IS NULL) OR (account_id IS NULL AND applicant_id IS NOT NULL))
);

COMMENT ON TABLE job_chat_message IS 'Append-only messages per D-030; 10 msg/min rate limit';

CREATE INDEX idx_job_chat_message_thread_id ON job_chat_message(thread_id);
CREATE INDEX idx_job_chat_message_created_at ON job_chat_message(created_at DESC);

-- Employee relations (created only by accepting an application)
CREATE TABLE employee_relation (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    job_application_id UUID REFERENCES job_application(id) ON DELETE SET NULL,
    provisioning_status TEXT DEFAULT 'pending', -- 'pending', 'provisioned', 'active'
    first_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    terminated_at TIMESTAMPTZ,
    
    UNIQUE (account_id, store_id),
    CHECK (provisioning_status IN ('pending', 'provisioned', 'active'))
);

COMMENT ON TABLE employee_relation IS 'Employee relations per store, created only by accepting job application';

CREATE INDEX idx_employee_relation_account_id ON employee_relation(account_id) WHERE terminated_at IS NULL;
CREATE INDEX idx_employee_relation_store_id ON employee_relation(store_id) WHERE terminated_at IS NULL;
