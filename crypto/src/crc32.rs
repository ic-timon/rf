//! # CRC32 校验和模块
//!
//! 提供 CRC32（循环冗余校验 32 位）校验和计算功能。
//!
//! CRC32 是一种快速的错误检测码，常用于数据传输和存储的完整性验证。
//! 它产生一个 32 位的校验值。
//!
//! # 使用示例
//!
//! ```ignore
//! use crypto::crc32::checksum;
//!
//! let data = b"Hello, world!";
//! let crc_value = checksum(data);
//! println!("CRC32: {:08x}", crc_value);
//! // 输出类似: CRC32: e7b45194
//! ```
//!
//! # 特点
//!
//! - 高速计算：CRC32 计算速度非常快，适合实时处理
//! // - 错误检测：可以检测到大多数数据传输错误
//! // - 不是加密哈希：不适合用于安全目的，只用于错误检测
//! //
//! // # 应用场景
//! //
//! // CRC32 适用于：
//! // - 文件完整性验证（如 ZIP、PNG 文件格式）
//! // - 网络数据传输错误检测
//! // - 存储系统数据校验
//! // - 快速数据去重
//! //
//! // @author TimonQWQ
//! // @date 2026-01-06

use crc32fast::Hasher;

/// 计算数据的 CRC32 校验和
///
/// 此函数对输入数据计算 CRC32 校验和，用于快速检测数据错误。
///
/// # 参数
///
/// * `data` - 要计算校验和的字节数据
///
/// # 返回值
///
/// 返回 32 位的无符号整数，表示 CRC32 校验和
///
/// # 使用示例
///
/// ```ignore
/// use crypto::crc32::checksum;
///
/// // 计算简单数据的校验和
/// let crc1 = checksum(b"Hello");
/// println!("CRC1: {:08x}", crc1);
///
/// // 计算文件的校验和
/// let file_content = std::fs::read("file.txt")?;
/// let crc2 = checksum(&file_content);
/// println!("File CRC: {:08x}", crc2);
///
/// // 验证数据完整性
/// let original_data = b"Important data";
/// let original_crc = checksum(original_data);
///
/// // 数据传输后重新计算
/// let received_data = original_data;
/// let received_crc = checksum(received_data);
///
/// if original_crc == received_crc {
///     println!("数据完整");
/// } else {
///     println!("数据损坏");
/// }
/// ```
///
/// # 性能说明
///
/// CRC32 使用硬件加速（如 SSE4.2 CRC32 指令）时性能非常高，
/// 每秒可以处理数 GB 的数据。这使得它非常适合用于实时系统。
///
/// # 与哈希算法的区别
///
/// - CRC32 是错误检测码，不是加密哈希函数
/// - CRC32 不具有抗碰撞性，不适合用于安全目的
/// - CRC32 计算速度远快于 SHA-256 等加密哈希
/// - 如果需要安全性，应该使用 SHA-256 等加密哈希算法
pub fn checksum(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

