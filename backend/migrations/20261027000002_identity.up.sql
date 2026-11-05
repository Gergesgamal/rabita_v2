-- backend/migrations/20261027000002_identity.up.sql

SET LOCAL search_path = app, public;

-- Account table (internal users: merchants, employees, admins, super admin)
CREATE TABLE account (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    identifier TEXT UNIQUE NOT NULL, -- e.g., "mer-a1b2c3d4e5f6g7h8"
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    
    CHECK (identifier ~ '^[a-z0-9-]+$'),
    CHECK (email ~ '^[^@]+@[^@]+\.[^@]+$')
);

COMMENT ON TABLE account IS 'Internal platform accounts: merchants, employees, admins, super admins';
COMMENT ON COLUMN account.identifier IS 'System-generated login identifier, e.g. mer-a1b2c3d4e5f6g7h8';
COMMENT ON COLUMN account.password_hash IS 'Argon2id hash, never logged or displayed after creation';

CREATE INDEX idx_account_email ON account(email) WHERE deleted_at IS NULL;
CREATE INDEX idx_account_identifier ON account(identifier) WHERE deleted_at IS NULL;
CREATE INDEX idx_account_deleted_at ON account(deleted_at);

-- Account roles
CREATE TABLE account_role (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    role_name TEXT NOT NULL, -- 'merchant', 'employee', 'admin', 'super_admin'
    context_id UUID NOT NULL, -- store_id for merchant/employee, admin context for admin
    store_id UUID, -- NULL for super_admin context
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    
    CHECK (role_name IN ('merchant', 'employee', 'admin', 'super_admin')),
    UNIQUE (account_id, role_name, context_id)
);

COMMENT ON TABLE account_role IS 'Bindings of accounts to roles in specific contexts (store or admin)';
COMMENT ON COLUMN account_role.context_id IS 'For merchants/employees: store_id; for admins: admin context uuid';

CREATE INDEX idx_account_role_account_id ON account_role(account_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_account_role_context_id ON account_role(context_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_account_role_deleted_at ON account_role(deleted_at);

-- Session table
CREATE TABLE session (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    context_id UUID NOT NULL, -- Current context (store_id or admin context)
    session_token TEXT UNIQUE NOT NULL,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    last_rotated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (expires_at > created_at)
);

COMMENT ON TABLE session IS 'Active user sessions with context isolation per D-004';
COMMENT ON COLUMN session.context_id IS 'Store_id or admin context; prevents cross-context privilege bleed';

CREATE INDEX idx_session_account_id ON session(account_id) WHERE revoked_at IS NULL;
CREATE INDEX idx_session_token ON session(session_token) WHERE revoked_at IS NULL;
CREATE INDEX idx_session_expires_at ON session(expires_at);

-- WebAuthn credentials for MFA
CREATE TABLE webauthn_credential (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    credential_id BYTEA NOT NULL UNIQUE,
    public_key BYTEA NOT NULL,
    sign_count BIGINT NOT NULL DEFAULT 0,
    is_backup_eligible BOOLEAN DEFAULT false,
    is_backup_state BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    CHECK (sign_count >= 0)
);

COMMENT ON TABLE webauthn_credential IS 'WebAuthn/FIDO2 credentials per D-013';

CREATE INDEX idx_webauthn_credential_account_id ON webauthn_credential(account_id);

-- TOTP secrets for 2FA fallback
CREATE TABLE totp_secret (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    secret_encrypted TEXT NOT NULL, -- KMS-envelope encrypted
    last_accepted_timestamp BIGINT, -- Prevents replay
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (account_id)
);

COMMENT ON TABLE totp_secret IS 'TOTP secrets with replay prevention via last-accepted-timestamp per D-047';
COMMENT ON COLUMN totp_secret.secret_encrypted IS 'Never logged or re-displayed after enrollment';

CREATE INDEX idx_totp_secret_account_id ON totp_secret(account_id);

-- Recovery codes for account recovery
CREATE TABLE recovery_code (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES account(id) ON DELETE CASCADE,
    code_hash TEXT NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT now(),
    
    UNIQUE (account_id, code_hash)
);

COMMENT ON TABLE recovery_code IS 'Single-use Argon2id-hashed recovery codes per D-048';

CREATE INDEX idx_recovery_code_account_id ON recovery_code(account_id);

-- Consumer identity (store-scoped, deliberately unlinked from account)
CREATE TABLE consumer (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID NOT NULL, -- Will be FK after store table is created
    normalized_email TEXT NOT NULL,
    email_verified_at TIMESTAMPTZ,
    password_hash TEXT,
    phone_number TEXT,
    whatsapp_verified_at TIMESTAMPTZ,
    full_name TEXT,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now(),
    deleted_at TIMESTAMPTZ,
    
    UNIQUE (store_id, normalized_email),
    CHECK (normalized_email ~ '^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$')
);

COMMENT ON TABLE consumer IS 'Store-scoped consumer identities per D-018; deliberately NOT linked to internal accounts';
COMMENT ON COLUMN consumer.normalized_email IS 'Lowercase, for (store_id, normalized_email) uniqueness';

CREATE INDEX idx_consumer_store_id ON consumer(store_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_consumer_email_verified_at ON consumer(email_verified_at);
CREATE INDEX idx_consumer_deleted_at ON consumer(deleted_at);
