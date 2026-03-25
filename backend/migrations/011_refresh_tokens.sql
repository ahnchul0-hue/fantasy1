-- Migration 011: Refresh tokens table (Refresh Token Rotation)
-- Stores hashed refresh tokens for secure token rotation and theft detection

CREATE TABLE IF NOT EXISTS refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(128) NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT false,
    replaced_by UUID REFERENCES refresh_tokens(id),
    -- Self-FK: points to the new token that replaced this one
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- Prevent self-referencing loops
    CONSTRAINT chk_no_self_replace CHECK (replaced_by IS NULL OR replaced_by != id)
);

-- Lookup by token hash (for validation)
CREATE UNIQUE INDEX IF NOT EXISTS idx_refresh_tokens_hash ON refresh_tokens (token_hash);

-- Cleanup: find all tokens for a user (for mass revocation)
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_user_id ON refresh_tokens (user_id);

-- Expiry cleanup
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_expires_at ON refresh_tokens (expires_at)
    WHERE revoked = false;
