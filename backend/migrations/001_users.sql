-- Migration 001: Users table
-- Idempotent: uses IF NOT EXISTS

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    provider VARCHAR(20) NOT NULL CHECK (provider IN ('kakao', 'apple', 'google')),
    provider_user_id VARCHAR(255) NOT NULL,
    nickname VARCHAR(100),
    has_profile BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at TIMESTAMPTZ,

    -- No table-level unique: partial index below handles active-only uniqueness
    CONSTRAINT chk_users_provider CHECK (provider IN ('kakao', 'apple', 'google'))
);

-- Unique only among active (non-deleted) users — allows re-signup after deletion
CREATE UNIQUE INDEX IF NOT EXISTS uq_users_active_provider
    ON users (provider, provider_user_id) WHERE deleted_at IS NULL;

-- Indexes
CREATE INDEX IF NOT EXISTS idx_users_provider ON users (provider);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users (created_at);
CREATE INDEX IF NOT EXISTS idx_users_deleted_at ON users (deleted_at) WHERE deleted_at IS NOT NULL;

-- Auto-update updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_users_updated_at ON users;
CREATE TRIGGER trigger_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
