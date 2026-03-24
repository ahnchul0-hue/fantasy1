-- Migration 002: Orders table (IAP payments)
-- Tracks in-app purchase orders verified via RevenueCat

CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    receipt_id VARCHAR(512) NOT NULL,
    product_id VARCHAR(100) NOT NULL,
    platform VARCHAR(10) NOT NULL CHECK (platform IN ('ios', 'android')),
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'verified', 'failed', 'refunded')),
    amount_krw INTEGER,
    verified_at TIMESTAMPTZ,
    refunded_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Duplicate receipt check (idempotency)
CREATE UNIQUE INDEX IF NOT EXISTS idx_orders_receipt_id ON orders (receipt_id);

-- User's order history
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders (user_id);

-- Lookup by status for admin/analytics
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders (status);
