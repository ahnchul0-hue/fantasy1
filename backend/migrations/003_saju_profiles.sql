-- Migration 003: Saju profiles table
-- Stores the user's permanent saju profile (사주 원국)
-- Birth data is encrypted (BYTEA), four pillars stored as individual columns for querying

CREATE TABLE IF NOT EXISTS saju_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- Encrypted birth data (AES-256-GCM via CryptoService)
    birth_year_enc BYTEA NOT NULL,
    birth_month_enc BYTEA NOT NULL,
    birth_day_enc BYTEA NOT NULL,
    calendar_type VARCHAR(10) NOT NULL CHECK (calendar_type IN ('solar', 'lunar')),
    is_leap_month BOOLEAN NOT NULL DEFAULT false,
    birth_hour VARCHAR(20),
    gender VARCHAR(10) NOT NULL CHECK (gender IN ('male', 'female')),
    birth_hmac VARCHAR(64) NOT NULL,

    -- Four pillars: individual columns for efficient querying
    year_heavenly_stem VARCHAR(10) NOT NULL,
    year_earthly_branch VARCHAR(10) NOT NULL,
    month_heavenly_stem VARCHAR(10) NOT NULL,
    month_earthly_branch VARCHAR(10) NOT NULL,
    day_heavenly_stem VARCHAR(10) NOT NULL,
    day_earthly_branch VARCHAR(10) NOT NULL,
    hour_heavenly_stem VARCHAR(10),
    hour_earthly_branch VARCHAR(10),

    -- Derived data
    oheng_balance JSONB NOT NULL,
    ilju_name VARCHAR(20) NOT NULL,
    ilju_hanja VARCHAR(10) NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_saju_profiles_user_id UNIQUE (user_id)
);

CREATE INDEX IF NOT EXISTS idx_saju_profiles_user_id ON saju_profiles (user_id);
CREATE INDEX IF NOT EXISTS idx_saju_profiles_ilju_name ON saju_profiles (ilju_name);
CREATE INDEX IF NOT EXISTS idx_saju_profiles_birth_hmac ON saju_profiles (birth_hmac);

DROP TRIGGER IF EXISTS trigger_saju_profiles_updated_at ON saju_profiles;
CREATE TRIGGER trigger_saju_profiles_updated_at
    BEFORE UPDATE ON saju_profiles
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
