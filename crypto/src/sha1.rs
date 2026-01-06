//! # SHA-1 哈希模块
//!
//! 提供 SHA-1（安全哈希算法 1）哈希计算功能。
//!
//! SHA-1 产生一个 160 位（20 字节）的哈希值，通常表示为 40 个十六进制字符。
//!
//! # 使用示例
//!
//! ```ignore
//! use crypto::sha1::hash;
//!
//! let data = b"Hello, world!";
//! let hash_value = hash(data);
//! println!("SHA1 hash: {}", hash_value);
//! // 输出类似: SHA1 hash: d3486ae9136e7856bc42212385ea797094475802
//! ```
//!
//! # 安全注意事项
//!
//! SHA-1 已被认为不再安全，容易受到碰撞攻击。建议在新项目中使用 SHA-256 或更安全的哈希算法。
//! 此模块主要用于兼容性目的。
//!
//! @author TimonQWQ
//! @date 2026-01-06

use sha1::{Sha1, Digest};

/// 计算数据的 SHA-1 哈希值
///
/// 此函数对输入数据计算 SHA-1 哈希值，返回十六进制字符串格式的结果。
///
/// # 参数
///
/// * `data` - 要计算哈希值的字节数据
///
/// # 返回值
///
/// 返回 40 个字符的十六进制字符串，表示 160 位的 SHA-1 哈希值
///
/// # 使用示例
///
/// ```ignore
/// use crypto::sha1::hash;
///
/// // 计算简单字符串的哈希
/// let hash1 = hash(b"Hello");
///
/// // 计算文件内容的哈希
/// let file_content = std::fs::read("file.txt")?;
/// let hash2 = hash(&file_content);
///
/// // 计算字节数组的哈希
/// let data = vec![1, 2, 3, 4, 5];
/// let hash3 = hash(&data);
///
/// println!("Hash 1: {}", hash1);
/// println!("Hash 2: {}", hash2);
/// println!("Hash 3: {}", hash3);
/// ```
///
/// # 性能说明
///
/// SHA-1 是一个快速的哈希算法，适合用于：
/// - 数据完整性验证
/// - 数字签名（如果不需要最高安全性）
/// - 快速数据去重
///
/// 但不推荐用于：
/// - 密码存储
/// - 安全敏感的加密应用
pub fn hash(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

