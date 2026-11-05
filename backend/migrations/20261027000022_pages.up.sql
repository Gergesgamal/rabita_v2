-- backend/migrations/20261027000022_pages.up.sql

SET LOCAL search_path = app, public;

-- Puck pages per D-033/D-100
CREATE TABLE puck_page (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    page_type TEXT NOT NULL, -- 'landing', 'custom'
    is_published BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    
    UNIQUE (store_id, slug),
    CHECK (slug ~ '^[a-z0-9-]+$'),
    CHECK (page_type IN ('landing', 'custom'))
);

COMMENT ON TABLE puck_page IS 'Puck-customizable pages per D-033/D-100; merchants use Puck editor at /editor';

CREATE INDEX idx_puck_page_store_id ON puck_page(store_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_puck_page_is_published ON puck_page(is_published) WHERE deleted_at IS NULL;

-- Page revisions per D-033
CREATE TABLE puck_page_revision (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    page_id UUID NOT NULL REFERENCES puck_page(id) ON DELETE CASCADE,
    author_id UUID REFERENCES account(id) ON DELETE SET NULL,
    content JSONB NOT NULL, -- Puck serialized content per D-033
    revision_number BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (page_id, revision_number),
    CHECK (revision_number > 0)
);

COMMENT ON TABLE puck_page_revision IS 'Revisions per D-033; enables rollback without erasing history';

CREATE INDEX idx_puck_page_revision_page_id ON puck_page_revision(page_id);

-- Page publication history per D-033
CREATE TABLE puck_page_publish (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    page_id UUID NOT NULL REFERENCES puck_page(id) ON DELETE CASCADE,
    revision_id UUID NOT NULL REFERENCES puck_page_revision(id) ON DELETE CASCADE,
    publisher_id UUID REFERENCES account(id) ON DELETE SET NULL,
    published_at TIMESTAMPTZ DEFAULT now(),
    rolled_back_at TIMESTAMPTZ,
    
    UNIQUE (page_id, revision_id)
);

COMMENT ON TABLE puck_page_publish IS 'Publication audit per D-033; rollback creates new publish record, not erasure';

CREATE INDEX idx_puck_page_publish_page_id ON puck_page_publish(page_id);

-- Preview tokens per D-033
CREATE TABLE page_preview_token (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    page_id UUID NOT NULL REFERENCES puck_page(id) ON DELETE CASCADE,
    revision_id UUID REFERENCES puck_page_revision(id) ON DELETE CASCADE,
    token_hash TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    
    CHECK (expires_at > created_at)
);

COMMENT ON TABLE page_preview_token IS 'Short-lived preview tokens per D-033; scoped to store/page/revision';

CREATE INDEX idx_page_preview_token_page_id ON page_preview_token(page_id);
CREATE INDEX idx_page_preview_token_expires_at ON page_preview_token(expires_at);
