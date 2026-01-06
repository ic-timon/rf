//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # RF 加密模块
//!
//! 提供加密和哈希功能的综合加密模块。
//!
//! 本模块包含以下子模块：
//! - `aes`: AES-256-GCM 加密（高级加密标准）
//! - `des`: DES 加密（数据加密标准）
//! - `rsa`: RSA 非对称加密
//! - `md5`: MD5 哈希算法
//! - `sha1`: SHA-1 哈希算法
//! - `sha256`: SHA-256 哈希算法
//! - `crc32`: CRC32 校验和算法
//!
//! # 组织结构
//!
//! 为了避免命名冲突，本模块采用子模块组织方式。
//! 使用特定的模块路径调用功能，例如：
//!
//! ```ignore
//! use crypto::aes::encrypt;
//! use crypto::md5::hash;
//! ```
//!
//! # 使用示例
//!
//! ## AES 加密示例
//!
//! ```ignore
//! let key = [0u8; 32];  // 256位密钥
//! let nonce = [0u8; 12]; // 96位nonce
//! let plaintext = b"Hello, world!";
//!
//! let encrypted = crypto::aes::encrypt(&key, &nonce, plaintext)?;
//! let decrypted = crypto::aes::decrypt(&key, &nonce, &encrypted)?;
//! ```
//!
//! ## RSA 加密示例
//!
//! ```ignore
//! // 生成RSA密钥对
//! let key_pair = crypto::rsa::RsaKeyPair::new(2048)?;
//!
//! // 加密
//! let data = b"Secret message";
//! let encrypted = crypto::rsa::encrypt(key_pair.public_key(), data)?;
//!
//! // 解密
//! let decrypted = crypto::rsa::decrypt(key_pair.private_key(), &encrypted)?;
//! ```
//!
//! ## 哈希计算示例
//!
//! ```ignore
//! // MD5 哈希
//! let md5_hash = crypto::md5::hash(b"Hello");
//!
//! // SHA256 哈希
//! let sha256_hash = crypto::sha256::hash(b"Hello");
//!
//! // CRC32 校验和
//! let checksum = crypto::crc32::checksum(b"Hello");
//! ```
//!
//! @author TimonQWQ
//! @date 2026-01-06

pub mod aes;
pub mod des;
pub mod md5;
pub mod sha1;
pub mod sha256;
pub mod rsa;
pub mod crc32;

// 注意：我们不使用 glob 重导出以避免以下冲突：
// - aes::encrypt/decrypt, des::encrypt/decrypt, rsa::encrypt/decrypt
// - md5::hash, sha1::hash, sha256::hash
// 用户应该通过模块路径访问函数：crypto::aes::encrypt 等

