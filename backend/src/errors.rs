use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Payment required: {0}")]
    PaymentRequired(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Database error: {0}")]
    Database(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "unauthorized", msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "forbidden", msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.clone()),
            AppError::RateLimitExceeded(msg) => {
                (StatusCode::TOO_MANY_REQUESTS, "rate_limit_exceeded", msg.clone())
            }
            AppError::PaymentRequired(msg) => {
                (StatusCode::PAYMENT_REQUIRED, "payment_required", msg.clone())
            }
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg.clone()),
            AppError::ExternalService(msg) => {
                // Log the full error but return generic message to client
                tracing::error!("External service error: {}", msg);
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "service_unavailable",
                    "External service temporarily unavailable".to_string(),
                )
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "An internal error occurred".to_string(),
                )
            }
            AppError::Database(msg) => {
                tracing::error!("Database error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "A database error occurred".to_string(),
                )
            }
        };

        let body = json!({
            "error": {
                "type": error_type,
                "message": message,
            }
        });

        (status, Json(body)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::Database(err.to_string()),
        }
    }
}
