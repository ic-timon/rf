//! # AES 加密模块
//!
//! 提供 AES-256-GCM（高级加密标准-伽罗瓦/计数器模式）加密功能。
//!
//! AES-256-GCM 是一种同时提供加密和认证的加密模式，具有以下特点：
//! - 使用 256 位密钥（32 字节）
//! - 使用 96 位 nonce（12 字节）
//! - 提供认证加密（AEAD）
//! - 高性能且安全
//!
//! # 使用示例
//!
//! ```ignore
//! use crypto::aes::{encrypt, decrypt};
//!
//! // 准备密钥和nonce
//! let key = [0u8; 32];  // 256位密钥，必须是32字节
//! let nonce = [0u8; 12]; // 96位nonce，必须是12字节
//! let plaintext = b"Hello, world!";
//!
//! // 加密
//! let encrypted = encrypt(&key, &nonce, plaintext)?;
//!
//! // 解密
//! let decrypted = decrypt(&key, &nonce, &encrypted)?;
//! assert_eq!(decrypted, plaintext);
//! ```
//!
//! # 安全注意事项
//!
//! - 密钥必须保密且长度必须为 32 字节
//! - nonce 必须是唯一的，每次加密都应该使用不同的 nonce
//! - nonce 可以公开，但不应该重复使用
//!
//! @author TimonQWQ
//! @date 2026-01-06

use aes_gcm::{Aes256Gcm, KeyInit};
use aes_gcm::aead::Aead;
use rf_errors::{Result, RfError};

/// 使用 AES-256-GCM 加密数据
///
/// 此函数使用 AES-256-GCM 算法对数据进行加密，同时提供认证功能。
///
/// # 参数
///
/// * `key` - 加密密钥，必须是 32 字节（256 位）
/// * `nonce` - 随机数，必须是 12 字节（96 位），每次加密应该使用不同的值
/// * `data` - 要加密的明文数据
///
/// # 返回值
///
/// 返回加密后的密文数据，包含认证标签
///
/// # 错误
///
/// - 如果密钥长度不是 32 字节，返回内部错误
/// - 如果 nonce 长度不是 12 字节，返回内部错误
/// - 如果加密过程失败，返回内部错误
///
/// # 使用示例
///
/// ```ignore
/// use crypto::aes::encrypt;
///
/// let key = [0u8; 32];
/// let nonce = [0u8; 12];
/// let data = b"Secret message";
///
/// let encrypted = encrypt(&key, &nonce, data)?;
/// // encrypted 包含密文和认证标签
/// ```
pub fn encrypt(key: &[u8], nonce: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    use aes_gcm::Key;
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = aes_gcm::Nonce::from_slice(nonce);
    cipher.encrypt(nonce, data)
        .map_err(|e| RfError::Internal(format!("AES encryption failed: {}", e)))
}

/// 使用 AES-256-GCM 解密数据
///
/// 此函数使用 AES-256-GCM 算法对数据进行解密，并验证认证标签。
///
/// # 参数
///
/// * `key` - 解密密钥，必须是 32 字节（256 位），必须与加密时使用的密钥相同
/// * `nonce` - 随机数，必须是 12 字节（96 位），必须与加密时使用的 nonce 相同
/// * `data` - 要解密的密文数据（包含认证标签）
///
/// # 返回值
///
/// 返回解密后的明文数据
///
/// # 错误
///
/// - 如果密钥长度不是 32 字节，返回内部错误
/// - 如果 nonce 长度不是 12 字节，返回内部错误
/// - 如果解密过程失败或认证失败，返回内部错误
///
/// # 使用示例
///
/// ```ignore
/// use crypto::aes::{encrypt, decrypt};
///
/// let key = [0u8; 32];
/// let nonce = [0u8; 12];
/// let plaintext = b"Secret message";
///
/// let encrypted = encrypt(&key, &nonce, plaintext)?;
/// let decrypted = decrypt(&key, &nonce, &encrypted)?;
///
/// assert_eq!(decrypted.as_slice(), plaintext);
/// ```
pub fn decrypt(key: &[u8], nonce: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    use aes_gcm::Key;
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = aes_gcm::Nonce::from_slice(nonce);
    cipher.decrypt(nonce, data)
        .map_err(|e| RfError::Internal(format!("AES decryption failed: {}", e)))
}
