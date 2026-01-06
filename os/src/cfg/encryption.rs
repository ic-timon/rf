//! # encryption
//!
//! encryption 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Configuration encryption

use rf_errors::{Result, RfError};
use std::collections::HashMap;

/// Configuration encryption/decryption trait
pub trait ConfigEncryption: Send + Sync {
    /// Encrypt a configuration value
    fn encrypt(&self, value: &str) -> Result<String>;
    
    /// Decrypt a configuration value
    fn decrypt(&self, value: &str) -> Result<String>;
}

/// AES encryption (simplified - would use actual AES implementation)
pub struct AesEncryption {
    _key: Vec<u8>,
}

impl AesEncryption {
    /// Create a new AES encryption
    pub fn new(key: &[u8]) -> Self {
        Self {
            _key: key.to_vec(),
        }
    }
}

impl ConfigEncryption for AesEncryption {
    fn encrypt(&self, value: &str) -> Result<String> {
        // Simplified - would use actual AES encryption
        // For now, just base64 encode as placeholder
        use base64::{Engine as _, engine::general_purpose};
        let encoded = general_purpose::STANDARD.encode(value.as_bytes());
        Ok(format!("encrypted:{}", encoded))
    }
    
    fn decrypt(&self, value: &str) -> Result<String> {
        // Simplified - would use actual AES decryption
        if let Some(encoded) = value.strip_prefix("encrypted:") {
            use base64::{Engine as _, engine::general_purpose};
            let decoded = general_purpose::STANDARD.decode(encoded)
                .map_err(|e| RfError::Config(format!("Failed to decrypt: {}", e)))?;
            String::from_utf8(decoded)
                .map_err(|e| RfError::Config(format!("Invalid UTF-8 in decrypted value: {}", e)))
        } else {
            Ok(value.to_string())
        }
    }
}

/// No-op encryption (for testing or when encryption is disabled)
pub struct NoOpEncryption;

impl ConfigEncryption for NoOpEncryption {
    fn encrypt(&self, value: &str) -> Result<String> {
        Ok(value.to_string())
    }
    
    fn decrypt(&self, value: &str) -> Result<String> {
        Ok(value.to_string())
    }
}

/// Encrypt configuration values
pub fn encrypt_config(config: &HashMap<String, String>, encryption: &dyn ConfigEncryption) -> Result<HashMap<String, String>> {
    let mut encrypted = HashMap::new();
    for (key, value) in config {
        let encrypted_value = encryption.encrypt(value)?;
        encrypted.insert(key.clone(), encrypted_value);
    }
    Ok(encrypted)
}

/// Decrypt configuration values
pub fn decrypt_config(config: &HashMap<String, String>, encryption: &dyn ConfigEncryption) -> Result<HashMap<String, String>> {
    let mut decrypted = HashMap::new();
    for (key, value) in config {
        let decrypted_value = encryption.decrypt(value)?;
        decrypted.insert(key.clone(), decrypted_value);
    }
    Ok(decrypted)
}

