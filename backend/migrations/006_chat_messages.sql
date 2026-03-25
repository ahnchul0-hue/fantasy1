-- Migration 006: Chat messages table for AI consultation conversations
-- Content is encrypted (BYTEA) consistent with other PII tables

DO $$ BEGIN
    CREATE TYPE chat_role AS ENUM ('user', 'assistant');
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consultation_id UUID NOT NULL REFERENCES consultations(id) ON DELETE CASCADE,
    role chat_role NOT NULL,
    content_enc BYTEA NOT NULL,
    -- Encrypted chat content (AES-256-GCM via CryptoService)
    token_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Retrieve chat history for a consultation in order
CREATE INDEX IF NOT EXISTS idx_chat_messages_consultation_id ON chat_messages (consultation_id, created_at ASC);

-- Note: idx_chat_messages_consultation_role removed — no current query uses it
