//! # str
//!
//! str 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 字符串工具模块
//!
//! 提供基础字符串操作功能的工具集。
//!
//! 本模块包含常用的字符串处理函数，包括字符串检查、分割、连接、
//! 大小写转换、修剪和替换等操作。这些函数是对 Rust 标准库字符串方法的
//! 简单封装，提供一致的接口。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_text::str::*;
//!
//! // 检查字符串是否为空
//! assert_eq!(is_empty(""), true);
//! assert_eq!(is_empty("hello"), false);
//!
//! // 检查字符串是否包含子串
//! assert_eq!(contains("hello world", "world"), true);
//!
//! // 分割字符串
//! let parts = split("a,b,c", ",");
//! assert_eq!(parts, vec!["a", "b", "c"]);
//!
//! // 连接字符串
//! let strings = vec!["hello", "world"];
//! assert_eq!(join(&strings, " "), "hello world");
//!
//! // 修剪字符串
//! assert_eq!(trim("  hello  "), "hello");
//!
//! // 大小写转换
//! assert_eq!(to_upper("hello"), "HELLO");
//! assert_eq!(to_lower("HELLO"), "hello");
//!
//! // 替换子串
//! assert_eq!(replace("hello world", "world", "rust"), "hello rust");
//! ```

/// 检查字符串是否为空
///
/// 判断给定的字符串是否为空字符串（长度为 0）。
///
/// # 参数
///
/// * `s` - 要检查的字符串切片
///
/// # 返回值
///
/// 如果字符串为空返回 `true`，否则返回 `false`
///
/// # 示例
///
/// ```rust
/// use rf_text::str::is_empty;
///
/// assert_eq!(is_empty(""), true);
/// assert_eq!(is_empty("hello"), false);
/// assert_eq!(is_empty("  "), false); // 空格不为空
/// ```
pub fn is_empty(s: &str) -> bool {
    s.is_empty()
}

/// 检查字符串是否包含指定的子串
///
/// 判断字符串 `s` 是否包含子串 `substr`。
///
/// # 参数
///
/// * `s` - 要搜索的字符串切片
/// * `substr` - 要查找的子串
///
/// # 返回值
///
/// 如果 `s` 包含 `substr` 返回 `true`，否则返回 `false`
///
/// # 示例
///
/// ```rust
/// use rf_text::str::contains;
///
/// assert_eq!(contains("hello world", "world"), true);
/// assert_eq!(contains("hello world", "rust"), false);
/// assert_eq!(contains("hello", ""), true); // 空子串总是包含
/// ```
pub fn contains(s: &str, substr: &str) -> bool {
    s.contains(substr)
}

/// 按照指定的分隔符分割字符串
///
/// 使用 `delimiter` 作为分隔符将字符串 `s` 分割成多个子串。
///
/// # 参数
///
/// * `s` - 要分割的字符串切片
/// * `delimiter` - 用作分隔符的字符串
///
/// # 返回值
///
/// 返回包含所有分割后子串的向量
///
/// # 注意
///
/// - 如果分隔符在字符串开头或结尾，结果会包含空字符串
/// - 如果有连续的分隔符，结果中也会包含空字符串
///
/// # 示例
///
/// ```rust
/// use rf_text::str::split;
///
/// assert_eq!(split("a,b,c", ","), vec!["a", "b", "c"]);
/// assert_eq!(split("a::b::c", "::"), vec!["a", "b", "c"]);
/// assert_eq!(split(",a,b,", ","), vec!["", "a", "b", ""]);
/// ```
pub fn split<'a>(s: &'a str, delimiter: &'a str) -> Vec<&'a str> {
    s.split(delimiter).collect()
}

/// 使用指定的分隔符连接多个字符串
///
/// 将字符串切片中的所有字符串用 `delimiter` 连接成一个字符串。
///
/// # 参数
///
/// * `strings` - 要连接的字符串切片
/// * `delimiter` - 用作分隔符的字符串
///
/// # 返回值
///
/// 返回连接后的新字符串
///
/// # 示例
///
/// ```rust
/// use rf_text::str::join;
///
/// let strings = vec!["hello", "world", "rust"];
/// assert_eq!(join(&strings, " "), "hello world rust");
/// assert_eq!(join(&strings, ","), "hello,world,rust");
/// assert_eq!(join(&strings, ""), "helloworldrust");
///
/// // 空切片返回空字符串
/// assert_eq!(join(&[], ","), "");
/// ```
pub fn join(strings: &[&str], delimiter: &str) -> String {
    strings.join(delimiter)
}

/// 修剪字符串两端的空白字符
///
/// 移除字符串开头和结尾的所有空白字符（包括空格、制表符、换行符等）。
///
/// # 参数
///
/// * `s` - 要修剪的字符串切片
///
/// # 返回值
///
/// 返回修剪后的字符串切片
///
/// # 示例
///
/// ```rust
/// use rf_text::str::trim;
///
/// assert_eq!(trim("  hello  "), "hello");
/// assert_eq!(trim("\thello\t"), "hello");
/// assert_eq!(trim("\nhello\n"), "hello");
/// assert_eq!(trim("  hello world  "), "hello world");
/// assert_eq!(trim(""), "");
/// ```
pub fn trim(s: &str) -> &str {
    s.trim()
}

/// 将字符串转换为大写
///
/// 将字符串中的所有字符转换为大写形式。
///
/// # 参数
///
/// * `s` - 要转换的字符串切片
///
/// # 返回值
///
/// 返回转换为大写的新字符串
///
/// # 注意
///
/// 对于某些字符，大写转换可能产生多个字符（例如德语 ß 变为 SS）。
///
/// # 示例
///
/// ```rust
/// use rf_text::str::to_upper;
///
/// assert_eq!(to_upper("hello"), "HELLO");
/// assert_eq!(to_upper("Hello World"), "HELLO WORLD");
/// assert_eq!(to_upper("123"), "123"); // 数字不受影响
/// ```
pub fn to_upper(s: &str) -> String {
    s.to_uppercase()
}

/// 将字符串转换为小写
///
/// 将字符串中的所有字符转换为小写形式。
///
/// # 参数
///
/// * `s` - 要转换的字符串切片
///
/// # 返回值
///
/// 返回转换为小写的新字符串
///
/// # 示例
///
/// ```rust
/// use rf_text::str::to_lower;
///
/// assert_eq!(to_lower("HELLO"), "hello");
/// assert_eq!(to_lower("Hello World"), "hello world");
/// assert_eq!(to_lower("123"), "123"); // 数字不受影响
/// ```
pub fn to_lower(s: &str) -> String {
    s.to_lowercase()
}

/// 替换字符串中的所有匹配子串
///
/// 将字符串 `s` 中所有出现的 `from` 子串替换为 `to` 子串。
///
/// # 参数
///
/// * `s` - 原始字符串切片
/// * `from` - 要被替换的子串
/// * `to` - 替换后的子串
///
/// # 返回值
///
/// 返回替换后的新字符串
///
/// # 注意
///
/// - 此函数会替换所有匹配的子串，不仅仅是第一个
/// - 如果 `from` 为空字符串，会在每个字符之间插入 `to`
///
/// # 示例
///
/// ```rust
/// use rf_text::str::replace;
///
/// assert_eq!(replace("hello world", "world", "rust"), "hello rust");
/// assert_eq!(replace("aaa", "a", "b"), "bbb");
/// assert_eq!(replace("hello", "x", "y"), "hello"); // 无匹配时不改变
/// ```
pub fn replace(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

