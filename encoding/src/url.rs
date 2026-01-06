// # url
//!
//! url 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06
//! # URL 编码/解码模块
//!
//! 提供 URL 编码和解码功能，用于处理 URL 中的特殊字符。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{url_encode, url_decode};
//!
//! let url = "https://example.com/path?key=value&other=123";
//! let encoded = url_encode(url);
//! let decoded = url_decode(&encoded).unwrap();
//! ```
//!
//! ## URL 编码规则
//!
//! - 保留字符：字母、数字、-、_、.、~
//! - 空格编码为 +
//! - 其他字符编码为 %XX 格式（十六进制）

use rf_errors::{Result, RfError};

/// URL 结构
///
/// 表示解析后的 URL。
#[derive(Debug, Clone)]
pub struct Url {
    /// 协议（如 http、https）
    pub scheme: Option<String>,
    /// 主机名
    pub host: Option<String>,
    /// 路径
    pub path: Option<String>,
    /// 查询参数
    pub query: Option<String>,
    /// 片段
    pub fragment: Option<String>,
}

impl Url {
    /// 创建新的 URL
    pub fn new() -> Self {
        Self {
            scheme: None,
            host: None,
            path: None,
            query: None,
            fragment: None,
        }
    }
}

/// 解析 URL 字符串
///
/// # 参数
///
/// * `s` - URL 字符串
///
/// # 返回值
///
/// 返回解析后的 URL 结构
///
/// # 示例
///
/// ```rust
/// use rf_encoding::url_parse;
///
/// let url = url_parse("https://example.com/path?key=value");
/// assert_eq!(url.scheme, Some("https".to_string()));
/// ```
pub fn parse(s: &str) -> Result<Url> {
    // 简单的 URL 解析实现
    let mut url = Url::new();
    let mut remaining = s;

    // 解析协议
    if let Some((scheme_part, rest)) = remaining.split_once("://") {
        url.scheme = Some(scheme_part.to_string());
        remaining = rest;
    }

    // 解析路径和查询
    let parts: Vec<&str> = remaining.split('/').collect();
    if parts.len() > 1 {
        url.host = parts.get(0).map(|s| s.to_string());
        url.path = if parts.len() > 1 {
            Some(format!("/{}", parts[1..].join("/")))
        } else {
            None
        };
    }

    // 解析查询参数和片段
    if let Some((path_part, query_part)) = remaining.split_once('?') {
        url.query = Some(query_part.to_string());
        url.path = Some(path_part.to_string());
    } else if let Some((path_part, fragment_part)) = remaining.split_once('#') {
        url.fragment = Some(fragment_part.to_string());
        url.path = Some(path_part.to_string());
    } else if url.path.is_none() {
        url.path = Some(remaining.to_string());
    }

    Ok(url)
}

/// 获取 URL 的协议
///
/// # 参数
///
/// * `url` - URL 结构
///
/// # 返回值
///
/// 返回协议字符串
pub fn scheme(url: &Url) -> Option<&str> {
    url.scheme.as_deref()
}

/// 获取 URL 的主机
///
/// # 参数
///
/// * `url` - URL 结构
///
/// # 返回值
///
/// 返回主机字符串
pub fn host(url: &Url) -> Option<&str> {
    url.host.as_deref()
}

/// 获取 URL 的路径
///
/// # 参数
///
/// * `url` - URL 结构
///
/// # 返回值
///
/// 返回路径字符串
pub fn path(url: &Url) -> Option<&str> {
    url.path.as_deref()
}

/// 获取 URL 的查询参数
///
/// # 参数
///
/// * `url` - URL 结构
///
/// # 返回值
///
/// 返回查询参数字符串
pub fn query(url: &Url) -> Option<&str> {
    url.query.as_deref()
}

/// 获取 URL 的片段
///
/// # 参数
///
/// * `url` - URL 结构
///
/// # 返回值
///
/// 返回片段字符串
pub fn fragment(url: &Url) -> Option<&str> {
    url.fragment.as_deref()
}

/// 解析查询参数字符串为键值对
///
/// # 参数
///
/// * `query` - 查询参数字符串
///
/// # 返回值
///
/// 返回解析后的键值对
///
/// # 示例
///
/// ```rust
/// use rf_encoding::url_parse_query;
///
/// let query = "key1=value1&key2=value2";
/// let params = url_parse_query(query);
/// assert_eq!(params["key1"], "value1");
/// ```
pub fn parse_query(query: &str) -> Result<std::collections::HashMap<String, String>> {
    let mut params = std::collections::HashMap::new();

    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            params.insert(key.to_string(), value.to_string());
        }
    }

    Ok(params)
}

/// 将键值对编码为查询参数字符串
///
/// # 参数
///
/// * `params` - 键值对
///
/// # 返回值
///
/// 返回编码后的查询参数字符串
pub fn encode_query(params: &std::collections::HashMap<String, String>) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&")
}

/// 十六进制字符转数字
///
/// 将十六进制字符转换为对应的数值。
///
/// # 参数
///
/// * `c` - 十六进制字符
///
/// # 返回值
///
/// 返回对应的数值（0-15），转换失败时返回错误
///
/// # 错误
///
/// - 当字符不是有效的十六进制字符时
fn hex_to_nibble(c: char) -> Result<u8> {
    let val = if c.is_ascii_digit() {
        c as u8 - b'0'
    } else if ('a'..='f').contains(&c) {
        c as u8 - b'a' + 10
    } else if ('A'..='F').contains(&c) {
        c as u8 - b'A' + 10
    } else {
        return Err(RfError::Serialization("Invalid hex character".to_string()));
    };

    Ok(val)
}

/// 十六进制数字转字符
fn hex_char(n: u8) -> char {
    if n < 10 {
        (b'0' + n) as char
    } else {
        (b'a' + n - 10) as char
    }
}

/// URL 编码字符串
///
/// 对字符串中的特殊字符进行百分比编码。
///
/// # 参数
///
/// * `s` - 要编码的字符串
///
/// # 返回值
///
/// 返回编码后的字符串
///
/// # 示例
///
/// ```rust
/// use rf_encoding::url_encode;
///
/// let encoded = url_encode("hello world");
/// assert_eq!(encoded, "hello%20world");
/// ```
pub fn url_encode(s: &str) -> String {
    let mut result = Vec::new();

    for c in s.chars() {
        if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
            let mut bytes = [0u8; 4];
            let encoded = c.encode_utf8(&mut bytes);
            for b in encoded.as_bytes() {
                result.push(*b);
            }
        } else {
            let mut bytes = [0u8; 4];
            let utf8_bytes = c.encode_utf8(&mut bytes);
            for b in utf8_bytes.as_bytes() {
                result.push(b'%');
                result.push(hex_char((b >> 4) as u8) as u8);
                result.push(hex_char((b & 0x0F) as u8) as u8);
            }
        }
    }

    String::from_utf8(result).unwrap_or_default()
}

/// URL 解码字符串
///
/// 将 URL 编码的字符串还原为原始字符串。
///
/// # 参数
///
/// * `s` - 要解码的字符串
///
/// # 返回值
///
/// 返回解码后的字符串，解码失败时返回错误
///
/// # 错误
///
/// - 当 URL 编码格式不正确时
/// - 当十六进制字符无效时
///
/// # 示例
///
/// ```rust
/// use rf_encoding::url_decode;
///
/// let encoded = "hello%20world";
/// let decoded = url_decode(encoded).unwrap();
/// assert_eq!(decoded, "hello world");
/// ```
pub fn url_decode(s: &str) -> Result<String> {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next().ok_or_else(|| RfError::Serialization("Invalid URL encoding".to_string()))?;
            let h2 = chars.next().ok_or_else(|| RfError::Serialization("Invalid URL encoding".to_string()))?;
            let byte = (hex_to_nibble(h1)? << 4) | hex_to_nibble(h2)?;
            result.push(byte);
        } else if c == '+' {
            result.push(b' ');
        } else {
            let mut bytes = [0u8; 4];
            let utf8_bytes = c.encode_utf8(&mut bytes);
            for b in utf8_bytes.as_bytes() {
                result.push(*b);
            }
        }
    }

    String::from_utf8(result)
        .map_err(|e| RfError::Serialization(format!("URL decode error: {}", e)))
}
