-- backend/migrations/20261027000004_auth_attempts.up.sql

SET LOCAL search_path = app, public;

-- Attempt tokens for login state machine per D-015
CREATE TABLE attempt_token (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    token_hash TEXT UNIQUE NOT NULL,
    account_id UUID REFERENCES account(id) ON DELETE CASCADE,
    state TEXT NOT NULL, -- 'ISSUED', 'PASSWORD_OK', 'ENROLLMENT_PENDING', 'MFA_OK', 'SESSION_ISSUED', 'EXPIRED', 'BURNED', 'FAILED'
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    transitioned_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (state IN ('ISSUED', 'PASSWORD_OK', 'ENROLLMENT_PENDING', 'MFA_OK', 'SESSION_ISSUED', 'EXPIRED', 'BURNED', 'FAILED'))
);

COMMENT ON TABLE attempt_token IS 'Login attempt state machine per D-015; authorizing terminal state is SESSION_ISSUED only';
COMMENT ON COLUMN attempt_token.state IS 'ISSUED → PASSWORD_OK → ENROLLMENT_PENDING → MFA_OK → SESSION_ISSUED or terminal non-auth states';

CREATE INDEX idx_attempt_token_hash ON attempt_token(token_hash);
CREATE INDEX idx_attempt_token_account_id ON attempt_token(account_id);
CREATE INDEX idx_attempt_token_expires_at ON attempt_token(expires_at);

-- Idempotency key store per D-130 (principal-scoped)
CREATE TABLE idempotency_key (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    principal_type TEXT NOT NULL, -- 'account', 'consumer', 'guest_session', 'service'
    principal_id TEXT NOT NULL,
    key TEXT NOT NULL,
    method TEXT NOT NULL, -- 'POST', 'PUT', 'PATCH', 'DELETE'
    path TEXT NOT NULL,
    payload_hash TEXT NOT NULL, -- SHA256 of request body
    response_status SMALLINT,
    response_body JSONB,
    created_at TIMESTAMPTZ DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    
    PRIMARY KEY (principal_type, principal_id, key),
    CHECK (principal_type IN ('account', 'consumer', 'guest_session', 'service')),
    CHECK (expires_at > created_at)
);

COMMENT ON TABLE idempotency_key IS 'Idempotency key store per D-130, principal-scoped to prevent cross-principal replay';
COMMENT ON COLUMN idempotency_key.principal_type IS 'account, consumer, guest_session, or service to prevent cross-principal key collision';

CREATE INDEX idx_idempotency_key_principal ON idempotency_key(principal_type, principal_id);
CREATE INDEX idx_idempotency_key_expires_at ON idempotency_key(expires_at);
