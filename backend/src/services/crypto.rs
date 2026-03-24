use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::errors::AppError;

type HmacSha256 = Hmac<Sha256>;

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
    /// Returns nonce + ciphertext concatenated.
    pub fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, AppError> {
        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|e| AppError::Internal(format!("Cipher init error: {}", e)))?;

        // Generate random 96-bit nonce
        let nonce_bytes: [u8; 12] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| AppError::Internal(format!("Encryption error: {}", e)))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Decrypt a value encrypted with encrypt().
    /// Input: nonce (12 bytes) + ciphertext.
    pub fn decrypt(&self, encrypted: &[u8]) -> Result<String, AppError> {
        if encrypted.len() < 12 {
            return Err(AppError::Internal("Invalid encrypted data".to_string()));
        }

        let cipher = Aes256Gcm::new_from_slice(&self.encryption_key)
            .map_err(|e| AppError::Internal(format!("Cipher init error: {}", e)))?;

        let nonce = Nonce::from_slice(&encrypted[..12]);
        let ciphertext = &encrypted[12..];

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| AppError::Internal(format!("Decryption error: {}", e)))?;

        String::from_utf8(plaintext)
            .map_err(|e| AppError::Internal(format!("UTF-8 decode error: {}", e)))
    }

    /// Generate an HMAC for a cache key (non-reversible identifier).
    pub fn hmac(&self, data: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(&self.hmac_secret)
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
