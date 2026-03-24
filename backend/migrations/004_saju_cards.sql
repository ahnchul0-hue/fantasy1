-- Migration 004: Saju cards table
-- Caches generated saju card results by birth_hmac for deduplication

CREATE TABLE IF NOT EXISTS saju_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    birth_hmac VARCHAR(64) NOT NULL,
    -- HMAC of normalized birth input for cache lookup (non-PII)
    ilju_name VARCHAR(20) NOT NULL,
    ilju_hanja VARCHAR(10) NOT NULL,
    keywords JSONB NOT NULL DEFAULT '[]'::jsonb,
    lucky_element VARCHAR(20) NOT NULL,
    image_url TEXT,
    share_url VARCHAR(64),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Primary lookup: find cached card by birth hmac
CREATE UNIQUE INDEX IF NOT EXISTS idx_saju_cards_birth_hmac ON saju_cards (birth_hmac);

-- Share URL lookup
CREATE INDEX IF NOT EXISTS idx_saju_cards_share_url ON saju_cards (share_url) WHERE share_url IS NOT NULL;

-- Cleanup of old cards
CREATE INDEX IF NOT EXISTS idx_saju_cards_created_at ON saju_cards (created_at);
