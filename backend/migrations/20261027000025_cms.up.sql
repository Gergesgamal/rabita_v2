-- backend/migrations/20261027000025_cms.up.sql

SET LOCAL search_path = app, public;

-- CMS pages for platform/legal content per D-105
CREATE TABLE cms_page (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    is_published BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (slug ~ '^[a-z0-9-]+$')
);

COMMENT ON TABLE cms_page IS 'Admin-managed platform pages per D-105 (Terms, Privacy, About, Contact, etc.)';

CREATE INDEX idx_cms_page_slug ON cms_page(slug);
CREATE INDEX idx_cms_page_is_published ON cms_page(is_published);

-- CMS page revisions
CREATE TABLE cms_page_revision (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    page_id UUID NOT NULL REFERENCES cms_page(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES account(id) ON DELETE SET NULL,
    content TEXT NOT NULL, -- Sanitized HTML per D-105
    state TEXT NOT NULL DEFAULT 'draft', -- 'draft', 'in_review', 'published', 'archived'
    published_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    revision_number BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (page_id, revision_number),
    CHECK (state IN ('draft', 'in_review', 'published', 'archived')),
    CHECK (revision_number > 0)
);

COMMENT ON TABLE cms_page_revision IS 'Versioned CMS content per D-105; publication requires step-up per D-005';

CREATE INDEX idx_cms_page_revision_page_id ON cms_page_revision(page_id);
CREATE INDEX idx_cms_page_revision_state ON cms_page_revision(state);
