//! # RSA 非对称加密模块
//!
//! 提供 RSA（Rivest-Shamir-Adleman）非对称加密功能。
//!
//! RSA 是一种广泛使用的非对称加密算法，使用公钥加密、私钥解密。
//! 本模块使用 OAEP（最优非对称加密填充）填充方案和 SHA-256 哈希函数。
//!
//! # 使用示例
//!
//! ```ignore
//! use crypto::rsa::{RsaKeyPair, encrypt, decrypt};
//!
//! // 生成 RSA 密钥对（2048 位）
//! let key_pair = RsaKeyPair::new(2048)?;
//!
//! // 加密：使用公钥
//! let plaintext = b"Secret message";
//! let encrypted = encrypt(key_pair.public_key(), plaintext)?;
//!
//! // 解密：使用私钥
//! let decrypted = decrypt(key_pair.private_key(), &encrypted)?;
//!
//! assert_eq!(decrypted.as_slice(), plaintext);
//! ```
//!
//! # 特点
//!
//! - 非对称加密：公钥用于加密，私钥用于解密
//! - 安全性：基于大整数分解难题
//! - 密钥长度：建议使用至少 2048 位的密钥
//! - 填充方案：使用 OAEP 填充，增强安全性
//!
//! # 应用场景
//!
//! RSA 适用于：
//! - 密钥交换
//! - 数字签名
//! - 小量数据加密（通常用于加密对称密钥）
//! - 证书体系
//!
//! # 注意事项
//!
//! - RSA 加密有数据长度限制，不能超过密钥长度减去填充开销
//! - 对于大量数据，应使用混合加密：RSA 加密对称密钥，对称加密数据
//! - 私钥必须妥善保管，泄露会导致严重安全问题
//!
//! @author TimonQWQ
//! @date 2026-01-06

use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;
use rf_errors::{Result, RfError};

/// RSA 密钥对
///
/// 包含 RSA 私钥和公钥的结构体，用于 RSA 加密和解密操作。
///
/// # 使用示例
///
/// ```ignore
/// use crypto::rsa::RsaKeyPair;
///
/// // 生成 2048 位的 RSA 密钥对
/// let key_pair = RsaKeyPair::new(2048)?;
///
/// // 获取公钥用于加密
/// let public_key = key_pair.public_key();
///
/// // 获取私钥用于解密
/// let private_key = key_pair.private_key();
/// ```
///
/// # 密钥长度建议
///
/// - 2048 位：目前安全的基本要求
/// - 3072 位：更高的安全性
/// - 4096 位：最高安全性，但性能较低
pub struct RsaKeyPair {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl RsaKeyPair {
    /// 生成新的 RSA 密钥对
    ///
    /// 此函数生成指定长度的 RSA 密钥对，包含私钥和公钥。
    ///
    /// # 参数
    ///
    /// * `bits` - 密钥长度（位数），建议至少 2048 位
    ///
    /// # 返回值
    ///
    /// 返回包含私钥和公钥的 `RsaKeyPair` 结构体
    ///
    /// # 错误
    ///
    /// - 如果密钥长度无效（小于 512 位），返回内部错误
    /// - 如果密钥生成失败，返回内部错误
    ///
    /// # 使用示例
    ///
    /// ```ignore
    /// use crypto::rsa::RsaKeyPair;
    ///
    /// // 生成 2048 位的密钥对（推荐）
    /// let key_pair = RsaKeyPair::new(2048)?;
    ///
    /// // 生成 4096 位的密钥对（更高安全性）
    /// let key_pair = RsaKeyPair::new(4096)?;
    /// ```
    ///
    /// # 性能说明
    ///
    /// 密钥生成是一个计算密集型操作：
    /// - 2048 位密钥：通常需要几百毫秒
    /// - 4096 位密钥：可能需要数秒
    /// 建议：密钥对可以生成后重复使用，无需频繁生成
    pub fn new(bits: usize) -> Result<Self> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .map_err(|e| RfError::Internal(format!("Failed to generate RSA key: {}", e)))?;
        let public_key = RsaPublicKey::from(&private_key);
        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// 获取私钥
    ///
    /// 返回 RSA 密钥对中的私钥引用，用于解密操作。
    ///
    /// # 返回值
    ///
    /// 返回私钥的引用
    ///
    /// # 使用示例
    ///
    /// ```ignore
    /// use crypto::rsa::{RsaKeyPair, decrypt};
    ///
    /// let key_pair = RsaKeyPair::new(2048)?;
    /// let private_key = key_pair.private_key();
    ///
    /// // 使用私钥解密
    /// let decrypted = decrypt(private_key, &encrypted_data)?;
    /// ```
    ///
    /// # 安全注意事项
    ///
    /// 私钥必须妥善保管，不要泄露给未授权方。
    pub fn private_key(&self) -> &RsaPrivateKey {
        &self.private_key
    }

    /// 获取公钥
    ///
    /// 返回 RSA 密钥对中的公钥引用，用于加密操作。
    ///
    /// # 返回值
    ///
    /// 返回公钥的引用
    ///
    /// # 使用示例
    ///
    /// ```ignore
    /// use crypto::rsa::{RsaKeyPair, encrypt};
    ///
    /// let key_pair = RsaKeyPair::new(2048)?;
    /// let public_key = key_pair.public_key();
    ///
    /// // 使用公钥加密
    /// let encrypted = encrypt(public_key, b"Secret message")?;
    /// ```
    ///
    /// # 注意事项
    ///
    /// 公钥可以自由分发，不需要保密。
    pub fn public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }
}

/// 使用 RSA 公钥加密数据
///
/// 此函数使用 RSA 公钥和 OAEP 填充方案加密数据。
///
/// # 参数
///
/// * `public_key` - RSA 公钥，用于加密
/// * `data` - 要加密的明文数据
///
/// # 返回值
///
/// 返回加密后的密文数据
///
/// # 错误
///
/// - 如果数据长度超过 RSA 密钥长度限制，返回内部错误
/// - 如果加密过程失败，返回内部错误
///
/// # 使用示例
///
/// ```ignore
/// use crypto::rsa::{RsaKeyPair, encrypt, decrypt};
///
/// let key_pair = RsaKeyPair::new(2048)?;
/// let plaintext = b"Secret message";
///
/// // 使用公钥加密
/// let encrypted = encrypt(key_pair.public_key(), plaintext)?;
///
/// // 使用私钥解密
/// let decrypted = decrypt(key_pair.private_key(), &encrypted)?;
/// assert_eq!(decrypted.as_slice(), plaintext);
/// ```
///
/// # 数据长度限制
///
/// RSA 加密有数据长度限制：
/// - 2048 位密钥：最多加密约 190 字节
/// - 4096 位密钥：最多加密约 470 字节
///
/// 对于大量数据，应该使用混合加密方案：
/// 1. 生成随机对称密钥（如 AES-256）
/// 2. 使用对称加密加密数据
/// 3. 使用 RSA 加密对称密钥
///
/// # 填充方案
///
/// 使用 OAEP（Optimal Asymmetric Encryption Padding）填充：
/// - 提供更好的安全性
/// - 使用 SHA-256 作为哈希函数
/// - 防止选择明文攻击
pub fn encrypt(public_key: &RsaPublicKey, data: &[u8]) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();
    public_key.encrypt(&mut rng, rsa::Oaep::new::<Sha256>(), data)
        .map_err(|e| RfError::Internal(format!("RSA encryption failed: {}", e)))
}

/// 使用 RSA 私钥解密数据
///
/// 此函数使用 RSA 私钥和 OAEP 填充方案解密数据。
///
/// # 参数
///
/// * `private_key` - RSA 私钥，用于解密
/// * `data` - 要解密的密文数据
///
/// # 返回值
///
/// 返回解密后的明文数据
///
/// # 错误
///
/// - 如果解密过程失败，返回内部错误
/// - 如果数据被篡改或使用错误的密钥，返回内部错误
///
/// # 使用示例
///
/// ```ignore
/// use crypto::rsa::{RsaKeyPair, encrypt, decrypt};
///
/// let key_pair = RsaKeyPair::new(2048)?;
/// let plaintext = b"Secret message";
///
/// // 加密
/// let encrypted = encrypt(key_pair.public_key(), plaintext)?;
///
/// // 解密
/// let decrypted = decrypt(key_pair.private_key(), &encrypted)?;
/// assert_eq!(decrypted.as_slice(), plaintext);
///
/// // 使用错误的私钥会失败
/// let wrong_key_pair = RsaKeyPair::new(2048)?;
/// let result = decrypt(wrong_key_pair.private_key(), &encrypted);
/// assert!(result.is_err());
/// ```
///
/// # 安全注意事项
///
/// - 私钥必须妥善保管，不要泄露
/// - 解密失败可能意味着数据被篡改
/// - 在安全敏感的场景中，应该记录解密失败事件
pub fn decrypt(private_key: &RsaPrivateKey, data: &[u8]) -> Result<Vec<u8>> {
    private_key.decrypt(rsa::Oaep::new::<Sha256>(), data)
        .map_err(|e| RfError::Internal(format!("RSA decryption failed: {}", e)))
}
