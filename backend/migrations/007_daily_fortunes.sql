-- Migration 007: Pre-generated daily fortunes table
-- Fortunes are generated nightly by a cron job for each ilju (60 combinations)

CREATE TABLE IF NOT EXISTS daily_fortunes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date DATE NOT NULL,
    ilju VARCHAR(4) NOT NULL,
    -- 일주: 2-char combo e.g. "갑자"
    fortune_text TEXT NOT NULL,
    lucky_color VARCHAR(30) NOT NULL,
    lucky_number INTEGER CHECK (lucky_number BETWEEN 1 AND 99),
    overall_score INTEGER NOT NULL CHECK (overall_score BETWEEN 1 AND 5),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Primary lookup: today's fortune for a specific ilju
CREATE UNIQUE INDEX IF NOT EXISTS idx_daily_fortunes_date_ilju ON daily_fortunes (date, ilju);

-- Cleanup of old fortunes
CREATE INDEX IF NOT EXISTS idx_daily_fortunes_date ON daily_fortunes (date);
