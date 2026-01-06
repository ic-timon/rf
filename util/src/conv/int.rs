//! # int
//!
//! int 模块 - 有符号整数类型转换工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 有符号整数类型转换工具模块
//!
//! 本模块提供了各种有符号整数类型的转换功能，包括：
//! - i8, i16, i32, i64, isize 等整数类型
//! - 从字符串解析为整数

use rf_errors::Result;

/// 将值转换为 i8 类型
///
/// i8 是 8 位有符号整数，范围为 -128 到 127。
///
/// # 参数
/// - `value`: 实现了 `TryInto<i8>` trait 的任意类型
///
/// # 返回值
/// - `Ok(i8)`: 转换成功
/// - `Err(RfError)`: 转换失败（如超出范围）
///
/// # 示例
/// ```ignore
/// use rf_util::conv::int;
///
/// assert_eq!(int::i8(100i16).unwrap(), 100);
/// assert!(int::i8(200i16).is_err()); // 超出 i8 范围
/// ```
pub fn i8<T: TryInto<i8>>(value: T) -> Result<i8> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to i8".to_string()))
}

/// 将值转换为 i16 类型
///
/// i16 是 16 位有符号整数，范围为 -32768 到 32767。
///
/// # 参数
/// - `value`: 实现了 `TryInto<i16>` trait 的任意类型
///
/// # 返回值
/// - `Ok(i16)`: 转换成功
/// - `Err(RfError)`: 转换失败（如超出范围）
///
/// # 示例
/// ```ignore
/// use rf_util::conv::int;
///
/// assert_eq!(int::i16(100i32).unwrap(), 100);
/// ```
pub fn i16<T: TryInto<i16>>(value: T) -> Result<i16> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to i16".to_string()))
}

/// 将值转换为 i32 类型
///
/// i32 是 32 位有符号整数，范围为 -2147483648 到 2147483647。
///
/// # 参数
/// - `value`: 实现了 `TryInto<i32>` trait 的任意类型
///
/// # 返回值
/// - `Ok(i32)`: 转换成功
/// - `Err(RfError)`: 转换失败（如超出范围）
///
/// # 示例
/// ```ignore
/// use rf_util::conv::int;
///
/// assert_eq!(int::i32(100i64).unwrap(), 100);
/// ```
pub fn i32<T: TryInto<i32>>(value: T) -> Result<i32> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to i32".to_string()))
}

/// 将值转换为 i64 类型
///
/// i64 是 64 位有符号整数，范围非常大。
///
/// # 参数
/// - `value`: 实现了 `TryInto<i64>` trait 的任意类型
///
/// # 返回值
/// - `Ok(i64)`: 转换成功
/// - `Err(RfError)`: 转换失败（如超出范围）
///
/// # 示例
/// ```ignore
/// use rf_util::conv::int;
///
/// assert_eq!(int::i64(100u32).unwrap(), 100);
/// ```
pub fn i64<T: TryInto<i64>>(value: T) -> Result<i64> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to i64".to_string()))
}

/// 将值转换为 isize 类型
///
/// isize 是指针大小的有符号整数，大小取决于平台架构。
///
/// # 参数
/// - `value`: 实现了 `TryInto<isize>` trait 的任意类型
///
/// # 返回值
/// - `Ok(isize)`: 转换成功
/// - `Err(RfError)`: 转换失败
///
/// # 示例
/// ```ignore
/// use rf_util::conv::int;
///
/// assert_eq!(int::isize(100i64).unwrap(), 100);
/// ```
pub fn isize<T: TryInto<isize>>(value: T) -> Result<isize> {
    value.try_into()
        .map_err(|_| rf_errors::RfError::Internal("Failed to convert to isize".to_string()))
}

/// 从字符串解析为 i64 类型
///
/// # 参数
/// - `s`: 要解析的字符串
///
/// # 返回值
/// - `Ok(i64)`: 解析成功
/// - `Err(RfError)`: 解析失败（如无效的数字格式）
///
/// # 示例
/// ```ignore
/// use rf_util::conv::int;
///
/// assert_eq!(int::i64_from_str("12345").unwrap(), 12345);
/// assert_eq!(int::i64_from_str("-12345").unwrap(), -12345);
/// assert!(int::i64_from_str("abc").is_err()); // 无效格式
/// ```
pub fn i64_from_str(s: &str) -> Result<i64> {
    s.parse::<i64>()
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to parse i64 from string: {}", e)))
}

/// 从字符串解析为 i32 类型
///
/// # 参数
/// - `s`: 要解析的字符串
///
/// # 返回值
/// - `Ok(i32)`: 解析成功
/// - `Err(RfError)`: 解析失败（如无效的数字格式）
///
/// # 示例
/// ```ignore
/// use rf_util::conv::int;
///
/// assert_eq!(int::i32_from_str("12345").unwrap(), 12345);
/// assert_eq!(int::i32_from_str("-12345").unwrap(), -12345);
/// assert!(int::i32_from_str("abc").is_err()); // 无效格式
/// ```
pub fn i32_from_str(s: &str) -> Result<i32> {
    s.parse::<i32>()
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to parse i32 from string: {}", e)))
}

