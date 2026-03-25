mod api;
mod auth;
mod errors;
mod models;
mod saju;
mod services;
mod state;

use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::auth::jwt::JwtManager;
use crate::auth::middleware::auth_middleware;
use crate::saju::{SajuAnalyzer, SajuEngine, SajuInterpreter};
use crate::services::{CacheService, ClaudeClient, CryptoService, NanoBananaClient, RevenueCatClient};
use crate::state::{AppConfig, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing (structured logging)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,saju_backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting saju-backend v{}", env!("CARGO_PKG_VERSION"));

    // Database connection pool
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let max_connections: u32 = std::env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "20".to_string())
        .parse()
        .unwrap_or(20);

    let db = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database connected and migrations applied");

    // Redis cache
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());
    let cache = CacheService::new(&redis_url).await?;

    // JWT Manager
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");
    let access_expiry: i64 = std::env::var("JWT_ACCESS_EXPIRY_SECS")
        .unwrap_or_else(|_| "900".to_string())
        .parse()
        .unwrap_or(900);
    let refresh_expiry: i64 = std::env::var("JWT_REFRESH_EXPIRY_SECS")
        .unwrap_or_else(|_| "2592000".to_string())
        .parse()
        .unwrap_or(2592000);
    let jwt = Arc::new(JwtManager::new(&jwt_secret, access_expiry, refresh_expiry));

    // Crypto service (PII encryption + HMAC)
    let encryption_key = std::env::var("PII_ENCRYPTION_KEY")
        .expect("PII_ENCRYPTION_KEY must be set");
    let hmac_secret = std::env::var("HMAC_SECRET")
        .expect("HMAC_SECRET must be set");
    let crypto = Arc::new(CryptoService::new(&encryption_key, &hmac_secret)?);

    // Claude API client
    let claude_api_key = std::env::var("CLAUDE_API_KEY")
        .expect("CLAUDE_API_KEY must be set");
    let claude_model = std::env::var("CLAUDE_MODEL")
        .unwrap_or_else(|_| "claude-opus-4-6-20250219".to_string());
    let claude_max_cost: f64 = std::env::var("CLAUDE_MAX_COST_PER_SESSION_USD")
        .unwrap_or_else(|_| "2.0".to_string())
        .parse()
        .unwrap_or(2.0);
    let claude_max_tokens: u32 = std::env::var("CLAUDE_MAX_TOKENS_PER_MESSAGE")
        .unwrap_or_else(|_| "2000".to_string())
        .parse()
        .unwrap_or(2000);
    let claude = Arc::new(ClaudeClient::new(
        claude_api_key,
        claude_model,
        claude_max_cost,
    ));

    // NanoBanana client
    let nb_api_key = std::env::var("NANOBANANA_API_KEY")
        .unwrap_or_else(|_| "".to_string());
    let nb_base_url = std::env::var("NANOBANANA_BASE_URL")
        .unwrap_or_else(|_| "https://api.nanobanana.com/v1".to_string());
    let nanobanana = Arc::new(NanoBananaClient::new(nb_api_key, nb_base_url));

    // RevenueCat client
    let rc_api_key = std::env::var("REVENUECAT_API_KEY")
        .unwrap_or_else(|_| "".to_string());
    let rc_webhook_secret = std::env::var("REVENUECAT_WEBHOOK_SECRET")
        .unwrap_or_else(|_| "".to_string());
    if rc_webhook_secret.is_empty() {
        tracing::warn!(
            "REVENUECAT_WEBHOOK_SECRET is not set — all webhook requests will be rejected. \
            Set this in production to process RevenueCat webhooks."
        );
    }
    let revenuecat = Arc::new(RevenueCatClient::new(rc_api_key, rc_webhook_secret));

    // Saju engine components
    let saju_engine = Arc::new(SajuEngine::new());
    let saju_analyzer = Arc::new(SajuAnalyzer::new());
    let saju_interpreter = Arc::new(SajuInterpreter::new(
        (*claude).clone(),
        claude_max_tokens,
    ));

    // App config
    let free_card_daily_limit: u32 = std::env::var("FREE_CARD_DAILY_LIMIT")
        .unwrap_or_else(|_| "3".to_string())
        .parse()
        .unwrap_or(3);
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID")
        .unwrap_or_else(|_| "".to_string());
    let apple_bundle_id = std::env::var("APPLE_BUNDLE_ID")
        .unwrap_or_else(|_| "".to_string());
    let cors_origins: Vec<String> = std::env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let config = Arc::new(AppConfig {
        free_card_daily_limit,
        google_client_id,
        apple_bundle_id,
        cors_allowed_origins: cors_origins.clone(),
        claude_max_tokens_per_message: claude_max_tokens,
        claude_max_cost_per_session_usd: claude_max_cost,
    });

    // Build shared state
    let app_state = AppState {
        db,
        cache,
        jwt,
        crypto,
        claude,
        nanobanana,
        revenuecat,
        saju_engine,
        saju_analyzer,
        saju_interpreter,
        config,
    };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(
            cors_origins
                .iter()
                .filter_map(|o| o.parse().ok())
                .collect::<Vec<_>>(),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        // Health check (no auth)
        .route("/health", get(api::health::health_check))
        // Share links (no auth, public)
        .route("/s/{id}", get(api::share::handle_share_link))
        .route("/s/{id}/meta", get(api::share::share_link_meta))
        // API v1 routes
        .nest("/v1", api_routes(app_state.clone()))
        // Global middleware
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(app_state);

    // Start server
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let addr = format!("{}:{}", host, port);
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// API v1 routes
fn api_routes(state: AppState) -> Router<AppState> {
    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/auth/login", post(api::auth::login))
        .route("/auth/refresh", post(api::auth::refresh))
        .route("/saju/card/{id}", get(api::saju::get_card))
        .route("/saju/compatibility", post(api::saju::compatibility_preview))
        .route("/payment/webhook", post(api::payment::revenuecat_webhook));

    // Optional auth routes (works with or without token)
    let optional_auth_routes = Router::new()
        .route("/saju/card", post(api::saju::create_card))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth::middleware::optional_auth_middleware,
        ));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/auth/delete-account", delete(api::auth::delete_account))
        .route("/saju/consultation", post(api::saju::create_consultation))
        .route(
            "/saju/consultation/{id}/status",
            get(api::saju::consultation_status),
        )
        .route(
            "/saju/consultation/{id}/chat",
            post(api::saju::send_chat_message),
        )
        .route("/fortune/daily", get(api::fortune::get_daily_fortune))
        .route("/profile", get(api::profile::get_profile))
        .route("/payment/verify", post(api::payment::verify_payment))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(optional_auth_routes)
        .merge(protected_routes)
}
