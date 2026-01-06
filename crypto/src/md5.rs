//! # MD5 哈希模块
//!
//! 提供 MD5（消息摘要算法 5）哈希计算功能。
//!
//! MD5 产生一个 128 位（16 字节）的哈希值，通常表示为 32 个十六进制字符。
//!
//! # 使用示例
//!
//! ```ignore
//! use crypto::md5::hash;
//!
//! let data = b"Hello, world!";
//! let hash_value = hash(data);
//! println!("MD5 hash: {}", hash_value);
//! // // 输出类似: MD5 hash: 5eb63bbbe01eeed093cb22bb8f5acdc3
//! ```
//!
//! # 安全注意事项
//! ///
//! /// **MD5 已经不再安全**，不建议在新项目中使用。主要问题：
//! /// - 容易受到碰撞攻击
//! /// - 存在选择前缀攻击
//! /// - 不适合用于密码存储或数字签名
//! /// - 已被 NIST（美国国家标准与技术研究院）废弃
//! ///
//! /// 推荐使用 SHA-256 或更安全的哈希算法替代。
//! /// 此模块主要用于兼容性目的，例如：
//! /// - 与遗留系统集成
//! /// - 非安全相关的数据去重
//! /// - 快速文件校验（非安全场景）
//! ///
//! /// # 特点
//! ///
//! /// - 快速计算：MD5 计算速度非常快
//! /// - 固定输出：无论输入多长，输出总是 128 位
//! /// - 雪崩效应：输入微小变化会导致输出巨大变化
//! /// - 单向性：从哈希值无法反推原始数据（但不抗碰撞）
//! ///
//! /// @author TimonQWQ
//! /// @date 2026-01-06

use md5::{Md5, Digest};

/// 计算数据的 MD5 哈希值
///
/// 此函数对输入数据计算 MD5 哈希值，返回十六进制字符串格式的结果。
///
/// # 参数
///
/// * `data` - 要计算哈希值的字节数据
///
/// # 返回值
///
/// 返回 32 个字符的十六进制字符串，表示 128 位的 MD5 哈希值
///
/// # 使用示例
///
/// ```ignore
/// use crypto::md5::hash;
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
/// // 快速数据去重
/// let files = vec![b"content1", b"content2", b"content1"];
/// let unique_hashes: std::collections::HashSet<_> = files.iter()
///     .map(|content| hash(content))
///     .collect();
/// println!("Unique files: {}", unique_hashes.len());
/// ```
///
/// # 应用场景（仅限非安全场景）
///
/// MD5 仅适用于：
/// - 数据去重（非安全场景）
/// - 快速文件校验（检测非恶意损坏）
/// - 与遗留系统兼容
/// - 哈希表索引
///
/// 不适用于：
/// - 密码存储
/// - 数字签名
/// - 安全认证
/// - 加密货币
///
/// # 性能说明
///
/// MD5 是一个非常快的哈希算法，适合用于：
/// - 大数据快速去重
/// - 缓存键计算
/// - 非安全性的数据校验
///
/// 但由于其安全性问题，不应该用于任何安全相关的场景。
pub fn hash(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

