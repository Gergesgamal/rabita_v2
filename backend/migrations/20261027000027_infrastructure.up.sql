-- backend/migrations/20261027000027_infrastructure.up.sql

SET LOCAL search_path = app, public;

-- Health check logs per D-125/D-126
CREATE TABLE health_check_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    endpoint TEXT NOT NULL, -- '/health', '/ready'
    status_code SMALLINT NOT NULL,
    response_time_ms BIGINT,
    dependencies JSONB, -- {'postgres': 'ok', 'valkey': 'failed'}
    created_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (status_code >= 100 AND status_code < 600),
    CHECK (response_time_ms >= 0)
);

COMMENT ON TABLE health_check_log IS 'Health check audit trail per D-126';

CREATE INDEX idx_health_check_log_endpoint ON health_check_log(endpoint);
CREATE INDEX idx_health_check_log_created_at ON health_check_log(created_at DESC);

-- Notification log per D-102/D-125
CREATE TABLE notification_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    notification_type TEXT NOT NULL, -- 'email', 'whatsapp', 'in_app', 'push'
    recipient_type TEXT NOT NULL, -- 'consumer', 'merchant', 'employee', 'admin'
    recipient_id UUID,
    store_id UUID REFERENCES store(id) ON DELETE SET NULL,
    subject TEXT,
    body TEXT,
    channel_response JSONB, -- SES message ID, WhatsApp message ID, etc.
    delivery_status TEXT, -- 'queued', 'sent', 'delivered', 'bounced', 'complained', 'failed'
    bounce_reason TEXT,
    complaint_reason TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    sent_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    
    CHECK (notification_type IN ('email', 'whatsapp', 'in_app', 'push')),
    CHECK (recipient_type IN ('consumer', 'merchant', 'employee', 'admin')),
    CHECK (delivery_status IN ('queued', 'sent', 'delivered', 'bounced', 'complained', 'failed'))
);

COMMENT ON TABLE notification_log IS 'Notification audit with channel response per D-102/D-125';

CREATE INDEX idx_notification_log_recipient_type_id ON notification_log(recipient_type, recipient_id);
CREATE INDEX idx_notification_log_store_id ON notification_log(store_id);
CREATE INDEX idx_notification_log_delivery_status ON notification_log(delivery_status);
CREATE INDEX idx_notification_log_created_at ON notification_log(created_at DESC);
