//! # basic
//!
//! basic 模块 - 基础类型转换工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 基础类型转换工具模块
//!
//! 本模块提供了常用的基础类型转换功能，包括：
//! - 字符串转换
//! - 字节转换
//! - 布尔值转换
//! - 基本数值类型转换

use rf_errors::Result;

/// 将值转换为字符串
///
/// # 参数
/// - `value`: 实现了 `ToString` trait 的任意类型
///
/// # 返回值
/// 返回值的字符串表示形式
///
/// # 示例
/// ```ignore
/// use rf_util::conv::basic;
///
/// assert_eq!(basic::string(123), "123");
/// assert_eq!(basic::string(3.14), "3.14");
/// assert_eq!(basic::string(true), "true");
/// ```
pub fn string<T: ToString>(value: T) -> String {
    value.to_string()
}

/// 将值转换为字节向量
///
/// # 参数
/// - `value`: 实现了 `AsRef<[u8]>` trait 的任意类型
///
/// # 返回值
/// 返回包含字节的向量
///
/// # 示例
/// ```ignore
/// use rf_util::conv::basic;
///
/// let bytes = basic::bytes(&[1, 2, 3, 4]);
/// assert_eq!(bytes, vec![1, 2, 3, 4]);
///
/// let str_bytes = basic::bytes("hello");
/// assert_eq!(str_bytes, b"hello".to_vec());
/// ```
pub fn bytes<T: AsRef<[u8]>>(value: T) -> Vec<u8> {
    value.as_ref().to_vec()
}

/// 将值转换为单字节（u8）
///
/// # 参数
/// - `value`: 实现了 `TryInto<u8>` trait 的任意类型
///
/// # 返回值
/// - `Ok(u8)`: 转换成功
/// - `Err(RfError)`: 转换失败
///
/// # 示例
/// ```ignore
/// use rf_util::conv::basic;
///
/// assert_eq!(basic::byte(42u8).unwrap(), 42);
/// assert!(basic::byte(256u16).is_err()); // 超出 u8 范围
/// ```
pub fn byte<T: TryInto<u8>>(value: T) -> Result<u8> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to byte".to_string()))
}

/// 将值转换为布尔值
///
/// # 参数
/// - `value`: 实现了 `TryInto<bool>` trait 的任意类型
///
/// # 返回值
/// - `Ok(bool)`: 转换成功
/// - `Err(RfError)`: 转换失败
///
/// # 示例
/// ```ignore
/// use rf_util::conv::basic;
///
/// assert_eq!(basic::bool(1).unwrap(), true);
/// assert_eq!(basic::bool(0).unwrap(), false);
/// ```
pub fn bool<T: TryInto<bool>>(value: T) -> Result<bool> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to bool".to_string()))
}

/// 从字符串转换为布尔值
///
/// 支持多种常见的布尔值表示形式：
/// - 真值: "true", "1", "yes", "on"
/// - 假值: "false", "0", "no", "off", ""（空字符串）
///
/// # 参数
/// - `s`: 要转换的字符串
///
/// # 返回值
/// 返回布尔值，不区分大小写
///
/// # 示例
/// ```ignore
/// use rf_util::conv::basic;
///
/// assert_eq!(basic::bool_from_str("true"), true);
/// assert_eq!(basic::bool_from_str("TRUE"), true);
/// assert_eq!(basic::bool_from_str("1"), true);
/// assert_eq!(basic::bool_from_str("yes"), true);
/// assert_eq!(basic::bool_from_str("on"), true);
///
/// assert_eq!(basic::bool_from_str("false"), false);
/// assert_eq!(basic::bool_from_str("0"), false);
/// assert_eq!(basic::bool_from_str("no"), false);
/// assert_eq!(basic::bool_from_str(""), false);
/// ```
pub fn bool_from_str(s: &str) -> bool {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" | "" => false,
        _ => false,
    }
}

