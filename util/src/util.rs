//! # util
//!
//! util 模块 - 通用工具函数
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 通用工具函数模块
//!
//! 本模块提供了一系列常用的工具函数，主要用于字符串处理和默认值处理。

//! // 检查字符串值是否为空
//! //
//! // # 参数
//! // - `value`: 实现了 `AsRef<str>` trait 的任意字符串类型
//! //
//! // # 返回值
//! // - `true`: 字符串为空
//! // - `false`: 字符串不为空
//! //
//! // # 示例
//! // ```ignore
//! // use rf_util::is_empty;
//! //
//! // assert_eq!(is_empty(""), true);
//! // assert_eq!(is_empty("hello"), false);
//! // ```
pub fn is_empty<T: AsRef<str>>(value: T) -> bool {
    value.as_ref().is_empty()
}

/// 检查字符串值是否不为空
///
/// # 参数
/// - `value`: 实现了 `AsRef<str>` trait 的任意字符串类型
///
/// # 返回值
/// - `true`: 字符串不为空
/// - `false`: 字符串为空
///
/// # 示例
/// ```ignore
/// use rf_util::is_not_empty;
///
/// assert_eq!(is_not_empty("hello"), true);
/// assert_eq!(is_not_empty(""), false);
/// ```
pub fn is_not_empty<T: AsRef<str>>(value: T) -> bool {
    !value.as_ref().is_empty()
}

/// 如果字符串为空，则返回默认值
///
/// # 参数
/// - `value`: 实现了 `AsRef<str>` trait 的任意字符串类型
/// - `default`: 默认值字符串
///
/// # 返回值
/// - 如果输入值为空，返回默认值
/// - 如果输入值不为空，返回输入值本身
///
/// # 示例
/// ```ignore
/// use rf_util::default_if_empty;
///
/// assert_eq!(default_if_empty("", "default"), "default");
/// assert_eq!(default_if_empty("hello", "default"), "hello");
/// ```
pub fn default_if_empty<T: AsRef<str>>(value: T, default: &str) -> String {
    if value.as_ref().is_empty() {
        default.to_string()
    } else {
        value.as_ref().to_string()
    }
}

