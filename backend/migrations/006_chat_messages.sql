-- Migration 006: Chat messages table for AI consultation conversations

DO $$ BEGIN
    CREATE TYPE chat_role AS ENUM ('user', 'assistant');
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    consultation_id UUID NOT NULL REFERENCES consultations(id) ON DELETE CASCADE,
    role chat_role NOT NULL,
    content TEXT NOT NULL,
    token_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Retrieve chat history for a consultation in order
CREATE INDEX IF NOT EXISTS idx_chat_messages_consultation_id ON chat_messages (consultation_id, created_at ASC);

-- Token usage analytics
CREATE INDEX IF NOT EXISTS idx_chat_messages_consultation_role ON chat_messages (consultation_id, role);
