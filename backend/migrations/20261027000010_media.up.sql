-- backend/migrations/20261027000010_media.up.sql

SET LOCAL search_path = app, public;

-- Media objects with quota enforcement per D-037/D-104
CREATE TABLE media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL REFERENCES store(id) ON DELETE CASCADE,
    s3_staging_key TEXT, -- Before scan verdict
    s3_production_key TEXT, -- After promotion
    s3_quarantine_key TEXT, -- If threats found
    original_filename TEXT NOT NULL,
    media_type TEXT NOT NULL, -- 'image', 'video', 'pdf', 'other'
    file_size_bytes BIGINT NOT NULL,
    guarduty_scan_status TEXT, -- 'pending', 'no_threats_found', 'threats_found', 'unsupported', 'access_denied', 'failed'
    scan_timestamp TIMESTAMPTZ,
    is_active BOOLEAN DEFAULT false, -- Only active media counts toward quota
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (media_type IN ('image', 'video', 'pdf', 'other')),
    CHECK (file_size_bytes > 0),
    CHECK (guarduty_scan_status IN ('pending', 'no_threats_found', 'threats_found', 'unsupported', 'access_denied', 'failed'))
);

COMMENT ON TABLE media IS 'Media objects per D-037 with GuardDuty scanning lifecycle per D-104';
COMMENT ON COLUMN media.guarduty_scan_status IS 'Controls promotion: only no_threats_found → active; threats_found → quarantine; others → hold';
COMMENT ON COLUMN media.is_active IS 'Only active media counts against quota; promotion from staging sets this true';

CREATE INDEX idx_media_store_id ON media(store_id);
CREATE INDEX idx_media_guarduty_scan_status ON media(guarduty_scan_status) WHERE guarduty_scan_status IN ('pending', 'unsupported', 'access_denied', 'failed');
CREATE INDEX idx_media_is_active ON media(is_active) WHERE is_active = true;
CREATE INDEX idx_media_created_at ON media(created_at DESC);

-- Media quota per store per D-037
CREATE TABLE media_quota (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL UNIQUE REFERENCES store(id) ON DELETE CASCADE,
    quota_bytes BIGINT NOT NULL DEFAULT 3221225472, -- 3 GB default per D-037
    used_bytes BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (quota_bytes > 0),
    CHECK (used_bytes >= 0),
    CHECK (used_bytes <= quota_bytes)
);

COMMENT ON TABLE media_quota IS 'Per-store media storage quota per D-037, adjustable globally and per-store';

CREATE INDEX idx_media_quota_store_id ON media_quota(store_id);
