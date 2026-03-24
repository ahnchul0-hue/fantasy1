-- Migration 010: Device-based rate limiting
-- Tracks API usage per device to enforce rate limits on free endpoints

CREATE TABLE IF NOT EXISTS rate_limits (
    id BIGSERIAL PRIMARY KEY,
    device_id VARCHAR(255) NOT NULL,
    endpoint VARCHAR(100) NOT NULL,
    request_count INTEGER NOT NULL DEFAULT 1,
    window_start TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_rate_limits_device_endpoint_window UNIQUE (device_id, endpoint, window_start)
);

-- Primary lookup: current rate for a device+endpoint
CREATE INDEX IF NOT EXISTS idx_rate_limits_device_endpoint ON rate_limits (device_id, endpoint, window_start DESC);

-- Cleanup: delete expired windows (run periodically)
CREATE INDEX IF NOT EXISTS idx_rate_limits_window_start ON rate_limits (window_start);

-- Function to check and increment rate limit atomically
CREATE OR REPLACE FUNCTION check_rate_limit(
    p_device_id VARCHAR,
    p_endpoint VARCHAR,
    p_max_requests INTEGER,
    p_window_seconds INTEGER
) RETURNS BOOLEAN AS $$
DECLARE
    v_window_start TIMESTAMPTZ;
    v_count INTEGER;
BEGIN
    v_window_start := date_trunc('second', now()) - (EXTRACT(EPOCH FROM now())::INTEGER % p_window_seconds) * INTERVAL '1 second';

    INSERT INTO rate_limits (device_id, endpoint, request_count, window_start)
    VALUES (p_device_id, p_endpoint, 1, v_window_start)
    ON CONFLICT (device_id, endpoint, window_start)
    DO UPDATE SET request_count = rate_limits.request_count + 1
    RETURNING request_count INTO v_count;

    RETURN v_count <= p_max_requests;
END;
$$ LANGUAGE plpgsql;
