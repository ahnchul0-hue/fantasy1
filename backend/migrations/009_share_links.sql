-- Migration 009: Share links table
-- Custom redirect service for deep linking (replaces Firebase Dynamic Links)

CREATE TABLE IF NOT EXISTS share_links (
    id VARCHAR(32) PRIMARY KEY,
    -- Short ID for shareable URLs (e.g., /s/{id})
    target_type VARCHAR(20) NOT NULL CHECK (target_type IN ('card', 'invite')),
    target_id UUID NOT NULL,
    click_count INTEGER NOT NULL DEFAULT 0,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Lookup by target for deduplication
CREATE INDEX IF NOT EXISTS idx_share_links_target ON share_links (target_type, target_id);

-- Analytics: popular links
CREATE INDEX IF NOT EXISTS idx_share_links_click_count ON share_links (click_count DESC);
