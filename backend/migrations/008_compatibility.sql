-- Migration 008: Compatibility results cache
-- Caches compatibility scores between two birth profiles
-- Birth data is encrypted (BYTEA) consistent with other tables

CREATE TABLE IF NOT EXISTS compatibility_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pair_hash VARCHAR(64) NOT NULL,
    -- HMAC of sorted(person1_birth_hmac, person2_birth_hmac) for order-independent lookup
    person1_birth_enc BYTEA NOT NULL,
    person2_birth_enc BYTEA NOT NULL,
    score INTEGER NOT NULL CHECK (score BETWEEN 0 AND 100),
    summary TEXT NOT NULL,
    person1_element VARCHAR(20) NOT NULL,
    person2_element VARCHAR(20) NOT NULL,
    detailed_analysis JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Cache lookup by pair hash
CREATE UNIQUE INDEX IF NOT EXISTS idx_compatibility_pair_hash ON compatibility_results (pair_hash);

-- Cleanup of old results
CREATE INDEX IF NOT EXISTS idx_compatibility_created_at ON compatibility_results (created_at);
