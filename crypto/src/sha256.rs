//! # SHA-256 哈希模块
//!
//! 提供 SHA-256（安全哈希算法 256 位）哈希计算功能。
//!
//! SHA-256 是 SHA-2 家族的一员，产生一个 256 位（32 字节）的哈希值，
//! 通常表示为 64 个十六进制字符。它是目前广泛使用且安全的哈希算法。
//!
//! # 使用示例
//!
//! ```ignore
//! use crypto::sha256::hash;
//!
//! let data = b"Hello, world!";
//! let hash_value = hash(data);
//! println!("SHA256 hash: {}", hash_value);
//! // 输出类似: SHA256 hash: 315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3
//! ```
//!
//! # 特点
//!
//! - 高安全性：抗碰撞性强，尚未发现有效的碰撞攻击
//! - 固定输出：无论输入多长，输出总是 256 位
//! - 雪崩效应：输入微小变化会导致输出巨大变化
//! - 单向性：从哈希值无法反推原始数据
//!
//! @author TimonQWQ
//! @date 2026-01-06

use sha2::{Sha256, Digest};

/// 计算数据的 SHA-256 哈希值
///
/// 此函数对输入数据计算 SHA-256 哈希值，返回十六进制字符串格式的结果。
///
/// # 参数
///
/// * `data` - 要计算哈希值的字节数据
///
/// # 返回值
///
/// 返回 64 个字符的十六进制字符串，表示 256 位的 SHA-256 哈希值
///
/// # 使用示例
///
/// ```ignore
/// use crypto::sha256::hash;
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
///
/// // 验证数据完整性
/// let original_data = b"Important data";
/// let stored_hash = hash(original_data);
/// // ... 之后可以重新计算哈希来验证数据是否被修改
/// ```
///
/// # 应用场景
///
/// SHA-256 适用于：
/// - 密码存储（配合盐值使用）
/// - 数字签名
/// - 数据完整性验证
/// - 区块链技术
/// - 证书指纹
/// - 文件校验
///
/// # 性能说明
///
/// SHA-256 的计算速度适中，在安全性和性能之间取得了良好平衡。
/// 对于大多数应用场景，性能是可以接受的。
pub fn hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

