use async_trait::async_trait;
use base64::Engine as _;
use serde::Deserialize;

use crate::errors::AppError;

/// Verified social login user info
#[derive(Debug, Clone)]
pub struct SocialUser {
    pub provider: String,
    pub provider_user_id: String,
    pub nickname: Option<String>,
}

#[async_trait]
pub trait SocialAuthVerifier: Send + Sync {
    async fn verify(&self, token: &str) -> Result<SocialUser, AppError>;
}

// ========================================
// Kakao Login Verifier
// ========================================
pub struct KakaoVerifier {
    client: reqwest::Client,
}

impl KakaoVerifier {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Deserialize)]
struct KakaoUserResponse {
    id: i64,
    kakao_account: Option<KakaoAccount>,
}

#[derive(Deserialize)]
struct KakaoAccount {
    profile: Option<KakaoProfile>,
}

#[derive(Deserialize)]
struct KakaoProfile {
    nickname: Option<String>,
}

#[async_trait]
impl SocialAuthVerifier for KakaoVerifier {
    async fn verify(&self, token: &str) -> Result<SocialUser, AppError> {
        let resp = self
            .client
            .get("https://kapi.kakao.com/v2/user/me")
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("Kakao API error: {}", e)))?;

        if !resp.status().is_success() {
            return Err(AppError::Unauthorized("Invalid Kakao token".to_string()));
        }

        let user_info: KakaoUserResponse = resp
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Kakao response parse error: {}", e)))?;

        let nickname = user_info
            .kakao_account
            .and_then(|a| a.profile)
            .and_then(|p| p.nickname);

        Ok(SocialUser {
            provider: "kakao".to_string(),
            provider_user_id: user_info.id.to_string(),
            nickname,
        })
    }
}

// ========================================
// Apple Login Verifier
// ========================================
pub struct AppleVerifier {
    client: reqwest::Client,
    /// Apple JWKS 공개 키 캐시 (실제 운영 시 TTL 기반 캐시 권장)
    apple_bundle_id: String,
}

impl AppleVerifier {
    pub fn new(apple_bundle_id: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            apple_bundle_id,
        }
    }
}

#[derive(Deserialize)]
struct AppleIdTokenClaims {
    sub: String,
    email: Option<String>,
    iss: Option<String>,
    aud: Option<String>,
    exp: Option<u64>,
}

#[derive(Deserialize)]
struct AppleJwks {
    keys: Vec<AppleJwk>,
}

#[derive(Deserialize)]
struct AppleJwk {
    kid: String,
    kty: String,
    alg: Option<String>,
    n: String,
    e: String,
}

#[async_trait]
impl SocialAuthVerifier for AppleVerifier {
    async fn verify(&self, token: &str) -> Result<SocialUser, AppError> {
        // Step 1: JWT 구조 검증
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(AppError::Unauthorized("Invalid Apple token format".to_string()));
        }

        // Step 2: JWT 헤더에서 kid 추출
        let header_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(parts[0])
            .map_err(|_| AppError::Unauthorized("Invalid Apple token header encoding".to_string()))?;
        let header: serde_json::Value = serde_json::from_slice(&header_bytes)
            .map_err(|_| AppError::Unauthorized("Invalid Apple token header".to_string()))?;
        let kid = header["kid"]
            .as_str()
            .ok_or_else(|| AppError::Unauthorized("Apple token missing kid".to_string()))?;
        let alg = header["alg"]
            .as_str()
            .ok_or_else(|| AppError::Unauthorized("Apple token missing alg".to_string()))?;

        if alg != "RS256" {
            return Err(AppError::Unauthorized(
                format!("Unsupported Apple token algorithm: {}", alg),
            ));
        }

        // Step 3: Apple JWKS에서 매칭되는 공개 키 가져오기
        let jwks: AppleJwks = self
            .client
            .get("https://appleid.apple.com/auth/keys")
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("Apple JWKS fetch error: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Apple JWKS parse error: {}", e)))?;

        let jwk = jwks
            .keys
            .iter()
            .find(|k| k.kid == kid)
            .ok_or_else(|| AppError::Unauthorized("Apple token kid not found in JWKS".to_string()))?;

        // Step 4: jsonwebtoken 크레이트로 RS256 서명 검증
        let decoding_key = jsonwebtoken::DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
            .map_err(|e| AppError::Unauthorized(format!("Apple key decode error: {}", e)))?;

        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_issuer(&["https://appleid.apple.com"]);
        validation.set_audience(&[&self.apple_bundle_id]);

        let token_data =
            jsonwebtoken::decode::<AppleIdTokenClaims>(token, &decoding_key, &validation)
                .map_err(|e| match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        AppError::Unauthorized("Apple token expired".to_string())
                    }
                    jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
                        AppError::Unauthorized("Apple token issuer invalid".to_string())
                    }
                    jsonwebtoken::errors::ErrorKind::InvalidAudience => {
                        AppError::Unauthorized("Apple token audience mismatch".to_string())
                    }
                    _ => AppError::Unauthorized(format!("Apple token validation failed: {}", e)),
                })?;

        let claims = token_data.claims;

        Ok(SocialUser {
            provider: "apple".to_string(),
            provider_user_id: claims.sub,
            nickname: claims.email.map(|e| {
                e.split('@').next().unwrap_or("user").to_string()
            }),
        })
    }
}

// ========================================
// Google Login Verifier
// ========================================
pub struct GoogleVerifier {
    client: reqwest::Client,
    client_id: String,
}

impl GoogleVerifier {
    pub fn new(client_id: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            client_id,
        }
    }
}

#[derive(Deserialize)]
struct GoogleTokenInfo {
    sub: String,
    email: Option<String>,
    name: Option<String>,
    aud: String,
}

#[async_trait]
impl SocialAuthVerifier for GoogleVerifier {
    async fn verify(&self, token: &str) -> Result<SocialUser, AppError> {
        let resp = self
            .client
            .get(format!(
                "https://oauth2.googleapis.com/tokeninfo?id_token={}",
                token
            ))
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("Google API error: {}", e)))?;

        if !resp.status().is_success() {
            return Err(AppError::Unauthorized("Invalid Google token".to_string()));
        }

        let token_info: GoogleTokenInfo = resp
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Google response parse error: {}", e)))?;

        if token_info.aud != self.client_id {
            return Err(AppError::Unauthorized(
                "Google token audience mismatch".to_string(),
            ));
        }

        Ok(SocialUser {
            provider: "google".to_string(),
            provider_user_id: token_info.sub,
            nickname: token_info.name.or(token_info.email),
        })
    }
}

/// Factory to get the right verifier
pub fn get_verifier(
    provider: &str,
    google_client_id: &str,
    apple_bundle_id: &str,
) -> Result<Box<dyn SocialAuthVerifier>, AppError> {
    match provider {
        "kakao" => Ok(Box::new(KakaoVerifier::new())),
        "apple" => Ok(Box::new(AppleVerifier::new(apple_bundle_id.to_string()))),
        "google" => Ok(Box::new(GoogleVerifier::new(google_client_id.to_string()))),
        _ => Err(AppError::BadRequest(format!(
            "Unsupported provider: {}",
            provider
        ))),
    }
}
