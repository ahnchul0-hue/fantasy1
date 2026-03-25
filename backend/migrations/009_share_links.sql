-- Migration 009: Share links table
-- Custom redirect service for deep linking (replaces Firebase Dynamic Links)

CREATE TABLE IF NOT EXISTS share_links (
    id VARCHAR(32) PRIMARY KEY,
    -- Short ID for shareable URLs (e.g., /s/{id})
    target_type VARCHAR(20) NOT NULL CHECK (target_type IN ('card', 'invite')),
    target_id UUID NOT NULL,
    click_count INTEGER NOT NULL DEFAULT 0,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- One share link per target (prevents duplicates)
    CONSTRAINT uq_share_links_target UNIQUE (target_type, target_id)
);

-- Note: FK on target_id is intentionally omitted because target_type is polymorphic
-- (card → saju_cards, invite → users). Application layer enforces referential integrity.

-- Analytics: popular links (only if click_count queries are actually used)
-- Removed: idx_share_links_click_count — causes write amplification on every click
