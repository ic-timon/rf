//! # base64
//!
//! base64 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # Base64 编码/解码模块
//!
//! 提供 Base64 编码和解码功能，用于二进制数据与文本之间的转换。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{base64_encode, base64_decode};
//!
//! // 编码
//! let encoded = base64_encode(b"Hello World");
//! assert_eq!(encoded, "SGVsbG8gV29ybGQ=");
//!
//! // 解码
//! let decoded = base64_decode(&encoded).unwrap();
//! assert_eq!(decoded, b"Hello World");
//! ```

use base64::{engine::general_purpose, Engine};
use rf_errors::{Result, RfError};

/// 将字节数据编码为 Base64 字符串
///
/// # 参数
///
/// * `data` - 要编码的字节数据
///
/// # 返回值
///
/// 返回 Base64 编码后的字符串
///
/// # 示例
///
/// ```rust
/// use rf_encoding::base64_encode;
///
/// let encoded = base64_encode(b"Hello");
/// assert_eq!(encoded, "SGVsbG8=");
/// ```
pub fn encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// 从 Base64 字符串解码为字节数据
///
/// # 参数
///
/// * `s` - Base64 编码的字符串
///
/// # 返回值
///
/// 返回解码后的字节数据，如果输入不是有效的 Base64 字符串则返回错误
///
/// # 错误
///
/// 当输入包含非 Base64 字符或格式不正确时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::base64_decode;
///
/// let decoded = base64_decode("SGVsbG8=").unwrap();
/// assert_eq!(decoded, b"Hello");
/// ```
pub fn decode(s: &str) -> Result<Vec<u8>> {
    general_purpose::STANDARD
        .decode(s)
        .map_err(|e| RfError::Serialization(format!("Base64 decode error: {}", e)))
}

