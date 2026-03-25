use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::errors::AppError;

type HmacSha256 = Hmac<Sha256>;

/// Magic byte prefix indicating versioned ciphertext format.
/// 0x76 = 'v' in ASCII. Chosen because it's unlikely to collide
/// meaningfully with legacy nonce-first format.
const VERSIONED_MAGIC: u8 = b'v';

/// Current encryption key version. Increment when rotating keys.
const CURRENT_KEY_VERSION: u8 = 1;

/// Cryptographic service for PII encryption and HMAC cache key generation.
#[derive(Clone)]
pub struct CryptoService {
    encryption_key: Vec<u8>,
    hmac_secret: Vec<u8>,
}

impl CryptoService {
    pub fn new(encryption_key_hex: &str, hmac_secret: &str) -> Result<Self, AppError> {
        let encryption_key = hex::decode(encryption_key_hex).map_err(|e| {
            AppError::Internal(format!("Invalid encryption key hex: {}", e))
        })?;

        if encryption_key.len() != 32 {
            return Err(AppError::Internal(
                "Encryption key must be 32 bytes (64 hex chars)".to_string(),
            ));
        }

        Ok(Self {
            encryption_key,
            hmac_secret: hmac_secret.as_bytes().to_vec(),
        })
    }

    /// Encrypt a plaintext value using AES-256-GCM.
    /// Returns magic(1) + version(1) + nonce(12) + ciphertext.
    pub fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, AppError> {
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|e| AppError::Internal(format!("Cipher init error: {}", e)))?;

        // Generate random 96-bit nonce
        let nonce_bytes: [u8; 12] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| AppError::Internal(format!("Encryption error: {}", e)))?;

        // Versioned format: magic + version + nonce + ciphertext
        let mut result = Vec::with_capacity(2 + nonce_bytes.len() + ciphertext.len());
        result.push(VERSIONED_MAGIC);
        result.push(CURRENT_KEY_VERSION);
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Decrypt a value encrypted with encrypt().
    /// Supports both versioned format (magic + version + nonce + ciphertext)
    /// and legacy format (nonce + ciphertext) for backwards compatibility.
    pub fn decrypt(&self, encrypted: &[u8]) -> Result<String, AppError> {
        // Minimum: nonce(12) + AES-GCM tag(16) = 28 for empty plaintext
        if encrypted.len() < 28 {
            return Err(AppError::Internal("Invalid encrypted data: too short".to_string()));
        }

        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|e| AppError::Internal(format!("Cipher init error: {}", e)))?;

        if encrypted[0] == VERSIONED_MAGIC {
            // Likely versioned format: magic(1) + version(1) + nonce(12) + ciphertext
            let version = encrypted[1];
            let payload = &encrypted[2..];

            if version == CURRENT_KEY_VERSION && payload.len() >= 28 {
                let (nonce_bytes, ciphertext) = payload.split_at(12);
                let nonce = Nonce::from_slice(nonce_bytes);
                match cipher.decrypt(nonce, ciphertext) {
                    Ok(plaintext) => {
                        return String::from_utf8(plaintext)
                            .map_err(|e| AppError::Internal(format!("UTF-8 decode error: {}", e)));
                    }
                    Err(_) => {
                        // Versioned decryption failed — first nonce byte may coincidentally
                        // be 'v'. Fall through to legacy decryption.
                        tracing::debug!("Versioned decryption failed, trying legacy format");
                    }
                }
            }
            // Fall through: try legacy in case first nonce byte happened to be 'v'
            self.decrypt_legacy(&cipher, encrypted)
        } else {
            // Legacy format (pre-versioning): nonce(12) + ciphertext
            self.decrypt_legacy(&cipher, encrypted)
        }
    }

    /// Decrypt data that was encrypted before key versioning was added.
    /// Legacy format: nonce(12 bytes) + ciphertext.
    fn decrypt_legacy(&self, cipher: &Aes256Gcm, encrypted: &[u8]) -> Result<String, AppError> {
        let nonce = Nonce::from_slice(&encrypted[..12]);
        let ciphertext = &encrypted[12..];

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| AppError::Internal(format!("Decryption error (legacy format): {}", e)))?;

        String::from_utf8(plaintext)
            .map_err(|e| AppError::Internal(format!("UTF-8 decode error: {}", e)))
    }

    /// Generate an HMAC for a cache key (non-reversible identifier).
    pub fn hmac(&self, data: &str) -> String {
        let mut mac = <HmacSha256 as Mac>::new_from_slice(&self.hmac_secret)
            .expect("HMAC can take key of any size");
        mac.update(data.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    /// Encrypt an integer value (for birth year/month/day).
    pub fn encrypt_int(&self, value: i32) -> Result<Vec<u8>, AppError> {
        self.encrypt(&value.to_string())
    }

    /// Decrypt an integer value.
    pub fn decrypt_int(&self, encrypted: &[u8]) -> Result<i32, AppError> {
        let s = self.decrypt(encrypted)?;
        s.parse::<i32>()
            .map_err(|e| AppError::Internal(format!("Int parse error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_crypto() -> CryptoService {
        CryptoService::new(
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "test-hmac-secret",
        )
        .unwrap()
    }

    #[test]
    fn test_encrypt_decrypt() {
        let crypto = test_crypto();
        let plaintext = "1990";
        let encrypted = crypto.encrypt(plaintext).unwrap();
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_versioned_format_prefix() {
        let crypto = test_crypto();
        let encrypted = crypto.encrypt("test").unwrap();
        // Should start with magic byte 'v' followed by version 1
        assert_eq!(encrypted[0], b'v');
        assert_eq!(encrypted[1], CURRENT_KEY_VERSION);
        // Total: magic(1) + version(1) + nonce(12) + ciphertext(>=16 for AES-GCM)
        assert!(encrypted.len() >= 30);
    }

    #[test]
    fn test_decrypt_legacy_format() {
        // Simulate legacy format: nonce(12) + ciphertext (no magic/version prefix)
        let crypto = test_crypto();
        let cipher = Aes256Gcm::new_from_slice(&crypto.encryption_key).unwrap();
        let nonce_bytes: [u8; 12] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, b"legacy-data".as_ref()).unwrap();

        let mut legacy_encrypted = nonce_bytes.to_vec();
        legacy_encrypted.extend_from_slice(&ciphertext);

        // decrypt() should handle legacy format via fallback
        let decrypted = crypto.decrypt(&legacy_encrypted).unwrap();
        assert_eq!(decrypted, "legacy-data");
    }

    #[test]
    fn test_encrypt_int() {
        let crypto = test_crypto();
        let encrypted = crypto.encrypt_int(1995).unwrap();
        let decrypted = crypto.decrypt_int(&encrypted).unwrap();
        assert_eq!(decrypted, 1995);
    }

    #[test]
    fn test_hmac_deterministic() {
        let crypto = test_crypto();
        let h1 = crypto.hmac("test-data");
        let h2 = crypto.hmac("test-data");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_hmac_different_inputs() {
        let crypto = test_crypto();
        let h1 = crypto.hmac("data1");
        let h2 = crypto.hmac("data2");
        assert_ne!(h1, h2);
    }
}
