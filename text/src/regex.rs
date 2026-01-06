//! # regex
//!
//! regex 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 正则表达式工具模块
//!
//! 提供基于正则表达式的文本处理功能。
//!
//! 本模块封装了 Rust 标准库 `regex` crate 的功能，提供简单易用的接口
//! 来进行正则表达式匹配、查找和替换操作。所有函数都返回 `Result` 类型，
//! 便于错误处理。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_text::regex::*;
//!
//! // 检查字符串是否匹配模式
//! let matched = is_match(r"\d+", "abc123def").unwrap();
//! assert!(matched);
//!
//! // 查找第一个匹配
//! let first = find(r"\d+", "abc123def456").unwrap();
//! assert_eq!(first, Some("123".to_string()));
//!
//! // 查找所有匹配
//! let all = find_all(r"\d+", "abc123def456").unwrap();
//! assert_eq!(all, vec!["123", "456"]);
//!
//! // 替换所有匹配
//! let result = replace(r"\d+", "abc123def456", "X").unwrap();
//! assert_eq!(result, "abcXdefX");
//! ```

use regex::Regex;
use rf_errors::{Result, RfError};

/// 检查字符串是否匹配给定的正则表达式模式
///
/// 判断文本 `text` 是否包含至少一个匹配正则表达式 `pattern` 的子串。
///
/// # 参数
///
/// * `pattern` - 正则表达式模式字符串
/// * `text` - 要检查的文本
///
/// # 返回值
///
/// * `Ok(bool)` - 如果文本匹配模式返回 `true`，否则返回 `false`
/// * `Err(RfError)` - 如果正则表达式模式无效，返回错误
///
/// # 错误
///
/// 如果 `pattern` 不是有效的正则表达式，会返回 `RfError::Internal` 错误。
///
/// # 示例
///
/// ```rust
/// use rf_text::regex::is_match;
///
/// // 匹配数字
/// assert!(is_match(r"\d+", "abc123def").unwrap());
/// assert!(!is_match(r"\d+", "abcdef").unwrap());
///
/// // 匹配邮箱
/// assert!(is_match(r"[\w.]+@[\w.]+", "user@example.com").unwrap());
///
/// // 无效的正则表达式
/// assert!(is_match(r"[invalid", "test").is_err());
/// ```
pub fn is_match(pattern: &str, text: &str) -> Result<bool> {
    let re = Regex::new(pattern)
        .map_err(|e| RfError::Internal(format!("Invalid regex pattern: {}", e)))?;
    Ok(re.is_match(text))
}

/// 查找字符串中第一个匹配正则表达式模式的子串
///
/// 在文本 `text` 中搜索第一个匹配正则表达式 `pattern` 的子串。
///
/// # 参数
///
/// * `pattern` - 正则表达式模式字符串
/// * `text` - 要搜索的文本
///
/// # 返回值
///
/// * `Ok(Some(String))` - 如果找到匹配，返回包含匹配文本的 `Some`
/// * `Ok(None)` - 如果没有找到匹配，返回 `None`
/// * `Err(RfError)` - 如果正则表达式模式无效，返回错误
///
/// # 错误
///
/// 如果 `pattern` 不是有效的正则表达式，会返回 `RfError::Internal` 错误。
///
/// # 示例
///
/// ```rust
/// use rf_text::regex::find;
///
/// // 查找第一个数字序列
/// assert_eq!(find(r"\d+", "abc123def456").unwrap(), Some("123".to_string()));
///
/// // 查找第一个单词
/// assert_eq!(find(r"\w+", "  hello world  ").unwrap(), Some("hello".to_string()));
///
/// // 无匹配的情况
/// assert_eq!(find(r"\d+", "abcdef").unwrap(), None);
///
/// // 提取邮箱
/// assert_eq!(
///     find(r"[\w.]+@[\w.]+", "Contact: user@example.com for info").unwrap(),
///     Some("user@example.com".to_string())
/// );
/// ```
pub fn find(pattern: &str, text: &str) -> Result<Option<String>> {
    let re = Regex::new(pattern)
        .map_err(|e| RfError::Internal(format!("Invalid regex pattern: {}", e)))?;
    Ok(re.find(text).map(|m| m.as_str().to_string()))
}

/// 查找字符串中所有匹配正则表达式模式的子串
///
/// 在文本 `text` 中搜索所有匹配正则表达式 `pattern` 的子串。
///
/// # 参数
///
/// * `pattern` - 正则表达式模式字符串
/// * `text` - 要搜索的文本
///
/// # 返回值
///
/// * `Ok(Vec<String>)` - 返回包含所有匹配文本的向量（按出现顺序）
/// * `Err(RfError)` - 如果正则表达式模式无效，返回错误
///
/// # 错误
///
/// 如果 `pattern` 不是有效的正则表达式，会返回 `RfError::Internal` 错误。
///
/// # 示例
///
/// ```rust
/// use rf_text::regex::find_all;
///
/// // 查找所有数字序列
/// assert_eq!(
///     find_all(r"\d+", "abc123def456ghi789").unwrap(),
///     vec!["123", "456", "789"]
/// );
///
/// // 查找所有单词
/// let words = find_all(r"\w+", "hello world rust").unwrap();
/// assert_eq!(words, vec!["hello", "world", "rust"]);
///
/// // 无匹配的情况
/// assert_eq!(find_all(r"\d+", "abcdef").unwrap(), vec![]);
///
/// // 查找所有邮箱
/// let text = "Contact: user1@example.com or user2@example.org";
/// let emails = find_all(r"[\w.]+@[\w.]+", text).unwrap();
/// assert_eq!(emails, vec!["user1@example.com", "user2@example.org"]);
/// ```
pub fn find_all(pattern: &str, text: &str) -> Result<Vec<String>> {
    let re = Regex::new(pattern)
        .map_err(|e| RfError::Internal(format!("Invalid regex pattern: {}", e)))?;
    Ok(re.find_iter(text).map(|m| m.as_str().to_string()).collect())
}

/// 使用替换文本替换所有匹配正则表达式模式的子串
///
/// 将文本 `text` 中所有匹配正则表达式 `pattern` 的子串替换为 `replacement`。
///
/// # 参数
///
/// * `pattern` - 正则表达式模式字符串
/// * `text` - 原始文本
/// * `replacement` - 替换字符串（可以包含捕获组引用，如 `$1`, `$2` 等）
///
/// # 返回值
///
/// * `Ok(String)` - 返回替换后的新字符串
/// * `Err(RfError)` - 如果正则表达式模式无效，返回错误
///
/// # 错误
///
/// 如果 `pattern` 不是有效的正则表达式，会返回 `RfError::Internal` 错误。
///
/// # 注意
///
/// - 替换字符串中可以使用 `$name` 或 `$number` 来引用正则表达式中的捕获组
/// - `$$` 会被替换为字面的 `$` 字符
/// - 所有匹配的子串都会被替换
///
/// # 示例
///
/// ```rust
/// use rf_text::regex::replace;
///
/// // 替换所有数字为 X
/// assert_eq!(replace(r"\d+", "abc123def456", "X").unwrap(), "abcXdefX");
///
/// // 隐藏手机号码中间四位
/// assert_eq!(
///     replace(r"(\d{3})\d{4}(\d{4})", "电话: 13812345678", "$1****$2").unwrap(),
///     "电话: 138****5678"
/// );
///
/// // 替换多个空格为单个空格
/// assert_eq!(replace(r" +", "hello    world", " ").unwrap(), "hello world");
///
/// // 使用捕获组交换单词
/// assert_eq!(
///     replace(r"(\w+)\s+(\w+)", "hello world", "$2 $1").unwrap(),
///     "world hello"
/// );
///
/// // 无匹配的情况返回原文本
/// assert_eq!(replace(r"\d+", "abcdef", "X").unwrap(), "abcdef");
/// ```
pub fn replace(pattern: &str, text: &str, replacement: &str) -> Result<String> {
    let re = Regex::new(pattern)
        .map_err(|e| RfError::Internal(format!("Invalid regex pattern: {}", e)))?;
    Ok(re.replace_all(text, replacement).to_string())
}

