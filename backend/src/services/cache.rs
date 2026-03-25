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

    /// Check and increment rate limit for a device (daily window, KST boundary).
    /// Returns (current_count, is_allowed).
    pub async fn check_rate_limit(
        &self,
        device_id: &str,
        action: &str,
        limit: u32,
    ) -> Result<(u32, bool), AppError> {
        let mut conn = self.redis.clone();
        // Use KST (UTC+9) for daily rate limit boundaries
        let kst = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let today = chrono::Utc::now().with_timezone(&kst).format("%Y-%m-%d").to_string();
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

    /// Check and increment rate limit using both device ID and client IP.
    /// Either exceeding the limit will block the request.
    /// Returns true if the request is allowed.
    pub async fn check_rate_limit_with_ip(
        &self,
        device_id: Option<&str>,
        client_ip: &str,
        endpoint: &str,
        max_requests: u32,
        window_seconds: u32,
    ) -> Result<bool, AppError> {
        // Check IP-based limit
        let ip_key = format!("rate:{}:ip:{}", endpoint, client_ip);
        let ip_count: u32 = self.increment_and_get(&ip_key, window_seconds).await?;
        if ip_count > max_requests {
            return Ok(false);
        }

        // Check device-based limit (if provided)
        if let Some(did) = device_id {
            let device_key = format!("rate:{}:device:{}", endpoint, did);
            let device_count: u32 = self.increment_and_get(&device_key, window_seconds).await?;
            if device_count > max_requests {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Increment a Redis key and return the new count, setting TTL on first increment.
    async fn increment_and_get(&self, key: &str, window_seconds: u32) -> Result<u32, AppError> {
        let mut conn = self.redis.clone();
        let count: u32 = conn
            .incr(key, 1u32)
            .await
            .map_err(|e| AppError::Internal(format!("Redis incr error: {}", e)))?;

        if count == 1 {
            conn.expire::<_, ()>(key, window_seconds as i64)
                .await
                .map_err(|e| AppError::Internal(format!("Redis expire error: {}", e)))?;
        }

        Ok(count)
    }

    /// Get current rate limit count.
    pub async fn get_rate_limit_count(
        &self,
        device_id: &str,
        action: &str,
    ) -> Result<u32, AppError> {
        let mut conn = self.redis.clone();
        let kst = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let today = chrono::Utc::now().with_timezone(&kst).format("%Y-%m-%d").to_string();
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
    /// Returns the lock token if acquired, or a Conflict error if already locked.
    /// Uses a 120s TTL to accommodate long Claude API responses (up to 60s).
    pub async fn acquire_session_lock(
        &self,
        consultation_id: &str,
    ) -> Result<String, AppError> {
        let mut conn = self.redis.clone();
        let lock_key = format!("lock:session:{}", consultation_id);
        let lock_token = uuid::Uuid::new_v4().to_string();

        // SET NX EX (set if not exists, with 120s TTL)
        let result: bool = redis::cmd("SET")
            .arg(&lock_key)
            .arg(&lock_token)
            .arg("NX")
            .arg("EX")
            .arg(120)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::Internal(format!("Redis lock error: {}", e)))?;

        if result {
            Ok(lock_token)
        } else {
            Err(AppError::Conflict(
                "다른 메시지가 처리 중입니다. 잠시 후 다시 시도해주세요.".into(),
            ))
        }
    }

    /// Extend the TTL of a session lock (call periodically during long operations).
    /// Only extends if we still own the lock. Returns true if extended, false if lost.
    pub async fn extend_session_lock(
        &self,
        consultation_id: &str,
        lock_token: &str,
    ) -> Result<bool, AppError> {
        let mut conn = self.redis.clone();
        let lock_key = format!("lock:session:{}", consultation_id);

        // Lua script for atomic compare-and-expire
        let script = redis::Script::new(
            r#"
            if redis.call("GET", KEYS[1]) == ARGV[1] then
                return redis.call("EXPIRE", KEYS[1], 120)
            else
                return 0
            end
            "#,
        );
        let extended: i32 = script
            .key(&lock_key)
            .arg(lock_token)
            .invoke_async(&mut conn)
            .await
            .map_err(|e| AppError::Internal(format!("Redis lock extend error: {}", e)))?;

        Ok(extended == 1)
    }

    /// Release a session mutex lock atomically.
    /// Only deletes the lock if it still holds our token (prevents releasing another request's lock).
    pub async fn release_session_lock(
        &self,
        consultation_id: &str,
        lock_token: &str,
    ) -> Result<(), AppError> {
        let mut conn = self.redis.clone();
        let lock_key = format!("lock:session:{}", consultation_id);

        // Lua script for atomic compare-and-delete
        let script = redis::Script::new(
            r#"
            if redis.call("GET", KEYS[1]) == ARGV[1] then
                return redis.call("DEL", KEYS[1])
            else
                return 0
            end
            "#,
        );
        let _: i32 = script
            .key(&lock_key)
            .arg(lock_token)
            .invoke_async(&mut conn)
            .await
            .map_err(|e| AppError::Internal(format!("Redis lock release error: {}", e)))?;

        Ok(())
    }
}
