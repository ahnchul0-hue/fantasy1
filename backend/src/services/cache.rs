use redis::AsyncCommands;

use crate::errors::AppError;

/// Redis-based cache service for saju cards, daily fortunes, and rate limiting.
#[derive(Clone)]
pub struct CacheService {
    redis: redis::aio::ConnectionManager,
}

impl CacheService {
    pub async fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| AppError::Internal(format!("Redis connection error: {}", e)))?;

        let conn = redis::aio::ConnectionManager::new(client)
            .await
            .map_err(|e| AppError::Internal(format!("Redis connection manager error: {}", e)))?;

        Ok(Self { redis: conn })
    }

    // ========================================
    // Saju Card Cache
    // ========================================

    /// Cache a saju card result by HMAC key (30 day TTL).
    pub async fn cache_saju_card(
        &self,
        hmac_key: &str,
        card_json: &str,
    ) -> Result<(), AppError> {
        let mut conn = self.redis.clone();
        let cache_key = format!("card:{}", hmac_key);
        conn.set_ex::<_, _, ()>(&cache_key, card_json, 30 * 24 * 3600)
            .await
            .map_err(|e| AppError::Internal(format!("Redis set error: {}", e)))?;
        Ok(())
    }

    /// Get a cached saju card by HMAC key.
    pub async fn get_cached_saju_card(
        &self,
        hmac_key: &str,
    ) -> Result<Option<String>, AppError> {
        let mut conn = self.redis.clone();
        let cache_key = format!("card:{}", hmac_key);
        let result: Option<String> = conn
            .get(&cache_key)
            .await
            .map_err(|e| AppError::Internal(format!("Redis get error: {}", e)))?;
        Ok(result)
    }

    // ========================================
    // Daily Fortune Cache
    // ========================================

    /// Cache a daily fortune (24h TTL).
    pub async fn cache_daily_fortune(
        &self,
        date: &str,
        ilju: &str,
        fortune_json: &str,
    ) -> Result<(), AppError> {
        let mut conn = self.redis.clone();
        let cache_key = format!("fortune:{}:{}", date, ilju);
        conn.set_ex::<_, _, ()>(&cache_key, fortune_json, 24 * 3600)
            .await
            .map_err(|e| AppError::Internal(format!("Redis set error: {}", e)))?;
        Ok(())
    }

    /// Get a cached daily fortune.
    pub async fn get_cached_daily_fortune(
        &self,
        date: &str,
        ilju: &str,
    ) -> Result<Option<String>, AppError> {
        let mut conn = self.redis.clone();
        let cache_key = format!("fortune:{}:{}", date, ilju);
        let result: Option<String> = conn
            .get(&cache_key)
            .await
            .map_err(|e| AppError::Internal(format!("Redis get error: {}", e)))?;
        Ok(result)
    }

    // ========================================
    // Rate Limiting
    // ========================================

    /// Check and increment rate limit for a device.
    /// Returns (current_count, is_allowed).
    pub async fn check_rate_limit(
        &self,
        device_id: &str,
        action: &str,
        limit: u32,
    ) -> Result<(u32, bool), AppError> {
        let mut conn = self.redis.clone();
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let cache_key = format!("ratelimit:{}:{}:{}", action, device_id, today);

        // Increment and get
        let count: u32 = conn
            .incr(&cache_key, 1u32)
            .await
            .map_err(|e| AppError::Internal(format!("Redis incr error: {}", e)))?;

        // Set TTL on first increment (24h)
        if count == 1 {
            conn.expire::<_, ()>(&cache_key, 24 * 3600)
                .await
                .map_err(|e| AppError::Internal(format!("Redis expire error: {}", e)))?;
        }

        let allowed = count <= limit;
        Ok((count, allowed))
    }

    /// Get current rate limit count.
    pub async fn get_rate_limit_count(
        &self,
        device_id: &str,
        action: &str,
    ) -> Result<u32, AppError> {
        let mut conn = self.redis.clone();
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let cache_key = format!("ratelimit:{}:{}:{}", action, device_id, today);

        let count: Option<u32> = conn
            .get(&cache_key)
            .await
            .map_err(|e| AppError::Internal(format!("Redis get error: {}", e)))?;

        Ok(count.unwrap_or(0))
    }

    // ========================================
    // Session Mutex (prevent concurrent chat messages)
    // ========================================

    /// Acquire a mutex lock for a consultation chat session.
    /// Returns true if lock acquired, false if already locked.
    pub async fn acquire_session_lock(
        &self,
        consultation_id: &str,
    ) -> Result<bool, AppError> {
        let mut conn = self.redis.clone();
        let lock_key = format!("lock:session:{}", consultation_id);

        // SET NX EX (set if not exists, with 30s TTL)
        let result: bool = redis::cmd("SET")
            .arg(&lock_key)
            .arg("locked")
            .arg("NX")
            .arg("EX")
            .arg(30)
            .query_async(&mut conn)
            .await
            .unwrap_or(false);

        Ok(result)
    }

    /// Release a session mutex lock.
    pub async fn release_session_lock(
        &self,
        consultation_id: &str,
    ) -> Result<(), AppError> {
        let mut conn = self.redis.clone();
        let lock_key = format!("lock:session:{}", consultation_id);
        conn.del::<_, ()>(&lock_key)
            .await
            .map_err(|e| AppError::Internal(format!("Redis del error: {}", e)))?;
        Ok(())
    }
}
