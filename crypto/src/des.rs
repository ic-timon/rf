//! # DES 加密模块
//!
//! 提供 DES（数据加密标准）对称加密功能。
//!
//! DES 是一种传统的对称加密算法，使用 56 位密钥（8 字节，其中 1 位奇偶校验位）。
//! 注意：DES 已被认为不再安全，容易被暴力破解。此模块主要用于兼容性目的。
//!
//! # 使用示例
//!
//! ```ignore
//! use crypto::des::{encrypt, decrypt};
//!
//! let key = [0u8; 8];  // 8字节密钥
//! let plaintext = b"Hello, world!";
//!
//! // 加密
//! let encrypted = encrypt(&key, plaintext)?;
//!
//! // 解密
//! let decrypted = decrypt(&key, &encrypted)?;
//! assert_eq!(decrypted, plaintext);
//! ```
//!
//! # 安全注意事项
//!
//! **DES 已经不再安全**，不建议在新项目中使用。主要问题：
//! - 密钥长度太短（56 位），容易受到暴力破解攻击
//! - 存在已知的密码分析攻击
//! - 已被 NIST（美国国家标准与技术研究院）废弃
//!
//! 推荐使用 AES-256 替代。
//!
//! # 特点
//!
//! - 对称加密：加密和解密使用相同的密钥
//! - 块大小：8 字节（64 位）
//! - 密钥长度：8 字节（有效密钥 56 位）
//! - 工作模式：电子密码本（ECB）模式
//!
//! // @author TimonQWQ
//! // @date 2026-01-06

use des::Des;
use cipher::{BlockEncrypt, BlockDecrypt, KeyInit};
use cipher::generic_array::GenericArray;
use rf_errors::{Result, RfError};

/// 使用 DES 加密数据
///
/// 此函数使用 DES 算法对数据进行加密。
///
/// # 参数
///
/// * `key` - 加密密钥，必须是 8 字节（64 位）
/// * `data` - 要加密的明文数据
///
/// # 返回值
///
/// 返回加密后的密文数据
///
/// # 错误
///
/// - 如果密钥长度不是 8 字节，返回内部错误
///
/// # 使用示例
///
/// ```ignore
/// use crypto::des::encrypt;
///
/// let key = [0u8; 8];
/// let plaintext = b"Secret message";
///
/// let encrypted = encrypt(&key, plaintext)?;
/// ```
///
/// # 块对齐说明
///
/// DES 是块密码，以 8 字节块为单位处理数据：
/// - 如果数据长度是 8 的倍数，完整加密
/// - 如果数据长度不是 8 的倍数，最后一个块会用零填充到 8 字节
///
/// 注意：本实现使用 ECB 模式，不提供认证功能。
///
/// # 性能说明
///
/// DES 加密速度较快，但安全性不足。仅在兼容性要求下使用。
pub fn encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 8 {
        return Err(RfError::Internal("DES key must be 8 bytes".to_string()));
    }

    let key = GenericArray::from_slice(key);
    let cipher = Des::new(key);

    // DES operates on 8-byte blocks
    let mut result = Vec::new();
    for chunk in data.chunks(8) {
        let mut block = GenericArray::clone_from_slice(&chunk.iter().cloned().chain(std::iter::repeat(0)).take(8).collect::<Vec<_>>());
        cipher.encrypt_block(&mut block);
        result.extend_from_slice(&block);
    }

    Ok(result)
}

/// 使用 DES 解密数据
///
/// 此函数使用 DES 算法对数据进行解密。
///
/// # 参数
///
/// * `key` - 解密密钥，必须是 8 字节（64 位），必须与加密时使用的密钥相同
/// * `data` - 要解密的密文数据
///
/// # 返回值
///
/// 返回解密后的明文数据
///
/// # 错误
///
/// - 如果密钥长度不是 8 字节，返回内部错误
///
/// # 使用示例
///
/// ```ignore
/// use crypto::des::{encrypt, decrypt};
///
/// let key = [0u8; 8];
/// let plaintext = b"Secret message";
///
/// let encrypted = encrypt(&key, plaintext)?;
/// let decrypted = decrypt(&key, &encrypted)?;
///
/// assert_eq!(decrypted, plaintext);
/// ```
///
/// # 数据长度说明
///
/// 输入数据长度必须是 8 的倍数，因为 DES 以 8 字节块工作。
/// 解密后的数据长度可能与原始明文不同（由于填充）。
pub fn decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 8 {
        return Err(RfError::Internal("DES key must be 8 bytes".to_string()));
    }

    let key = GenericArray::from_slice(key);
    let cipher = Des::new(key);

    let mut result = Vec::new();
    for chunk in data.chunks(8) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        result.extend_from_slice(&block);
    }

    Ok(result)
}
