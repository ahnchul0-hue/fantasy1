use sqlx::PgPool;
use std::sync::Arc;

use crate::auth::jwt::JwtManager;
use crate::services::{CacheService, ClaudeClient, CryptoService, NanoBananaClient, RevenueCatClient};
use crate::saju::{SajuAnalyzer, SajuEngine, SajuInterpreter};

/// Shared application state, passed to all handlers via Axum's State extractor.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cache: CacheService,
    pub jwt: Arc<JwtManager>,
    pub crypto: Arc<CryptoService>,
    pub claude: Arc<ClaudeClient>,
    pub nanobanana: Arc<NanoBananaClient>,
    pub revenuecat: Arc<RevenueCatClient>,
    pub saju_engine: Arc<SajuEngine>,
    pub saju_analyzer: Arc<SajuAnalyzer>,
    pub saju_interpreter: Arc<SajuInterpreter>,
    pub config: Arc<AppConfig>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub free_card_daily_limit: u32,
    pub google_client_id: String,
    pub apple_bundle_id: String,
    pub cors_allowed_origins: Vec<String>,
    pub claude_max_tokens_per_message: u32,
    pub claude_max_cost_per_session_usd: f64,
}
