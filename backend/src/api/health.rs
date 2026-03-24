use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::state::AppState;

/// GET /health
pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    // Check database connectivity
    let db_ok = sqlx::query("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    let status = if db_ok { "healthy" } else { "degraded" };

    Json(json!({
        "status": status,
        "version": env!("CARGO_PKG_VERSION"),
        "database": if db_ok { "connected" } else { "disconnected" },
    }))
}
