-- Migration 005: Paid consultations table

DO $$ BEGIN
    CREATE TYPE consultation_status AS ENUM ('generating', 'ready', 'expired', 'failed');
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE checkpoint_status AS ENUM ('none', 'analysis_done', 'images_done', 'complete');
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS consultations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES orders(id),
    birth_data_enc BYTEA NOT NULL,
    -- Encrypted birth input (AES-256-GCM)
    four_pillars JSONB NOT NULL,
    analysis_data JSONB NOT NULL,
    status consultation_status NOT NULL DEFAULT 'generating',
    checkpoint_status checkpoint_status NOT NULL DEFAULT 'none',
    analysis_summary TEXT,
    result_images JSONB NOT NULL DEFAULT '[]'::jsonb,
    consultation_type VARCHAR(30) NOT NULL DEFAULT 'saju_consultation'
        CHECK (consultation_type IN ('saju_consultation', 'compatibility_consultation')),
    chat_turns_remaining INTEGER NOT NULL DEFAULT 50
        CHECK (chat_turns_remaining >= 0),
    chat_turns_used INTEGER NOT NULL DEFAULT 0
        CHECK (chat_turns_used >= 0),
    chat_context JSONB NOT NULL DEFAULT '{}'::jsonb,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (now() + INTERVAL '72 hours'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- User's consultation history
CREATE INDEX IF NOT EXISTS idx_consultations_user_id ON consultations (user_id);
CREATE INDEX IF NOT EXISTS idx_consultations_user_status ON consultations (user_id, status);

-- Order lookup (idempotency check)
CREATE UNIQUE INDEX IF NOT EXISTS idx_consultations_order_id ON consultations (order_id);

-- Expiry cleanup
CREATE INDEX IF NOT EXISTS idx_consultations_expires_at ON consultations (expires_at)
    WHERE status IN ('generating', 'ready');

DROP TRIGGER IF EXISTS trigger_consultations_updated_at ON consultations;
CREATE TRIGGER trigger_consultations_updated_at
    BEFORE UPDATE ON consultations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
