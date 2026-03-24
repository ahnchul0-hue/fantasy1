use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: i64,
    pub iat: i64,
    pub token_type: String, // "access" or "refresh"
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_expiry_secs: i64,
    refresh_expiry_secs: i64,
}

impl JwtManager {
    pub fn new(secret: &str, access_expiry_secs: i64, refresh_expiry_secs: i64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_expiry_secs,
            refresh_expiry_secs,
        }
    }

    pub fn create_access_token(&self, user_id: Uuid) -> Result<String, AppError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::seconds(self.access_expiry_secs)).timestamp(),
            iat: now.timestamp(),
            token_type: "access".to_string(),
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(format!("JWT encode error: {}", e)))
    }

    pub fn create_refresh_token(&self, user_id: Uuid) -> Result<String, AppError> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::seconds(self.refresh_expiry_secs)).timestamp(),
            iat: now.timestamp(),
            token_type: "refresh".to_string(),
        };
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(format!("JWT encode error: {}", e)))
    }

    pub fn validate_access_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::Unauthorized("Token expired".to_string())
                }
                _ => AppError::Unauthorized(format!("Invalid token: {}", e)),
            })?;

        if token_data.claims.token_type != "access" {
            return Err(AppError::Unauthorized("Not an access token".to_string()));
        }

        Ok(token_data.claims)
    }

    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::Unauthorized("Refresh token expired".to_string())
                }
                _ => AppError::Unauthorized(format!("Invalid refresh token: {}", e)),
            })?;

        if token_data.claims.token_type != "refresh" {
            return Err(AppError::Unauthorized("Not a refresh token".to_string()));
        }

        Ok(token_data.claims)
    }

    pub fn access_expiry_secs(&self) -> i64 {
        self.access_expiry_secs
    }

    /// Hash a refresh token for secure storage
    pub fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }
}
