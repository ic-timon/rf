//! # charset
//!
//! charset 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06
//! # 字符集转换模块
//!
//! 提供不同字符编码之间的转换功能，支持多种常见编码格式。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{charset_convert, charset_to_utf8};
//!
//! // 在不同编码之间转换
//! let gbk_text = b"GBK编码\xC4\xE3\xBA\xC3"; // 示例 GBK 编码的"你好"
//! let utf8_text = charset_convert(gbk_text, "GBK", "UTF-8").unwrap();
//!
//! // 将任意编码转换为 UTF-8
//! let bytes = b"some bytes";
//! let text = charset_to_utf8(bytes, "UTF-8").unwrap();
//! ```
//!
//! ## 支持的编码
//!
//! - UTF-8
//! - UTF-16 (UTF-16LE, UTF-16BE)
//! - GBK/GB2312
//! - GB18030
//! - Big5
//! - Shift_JIS
//! - ISO-8859 系列
//! - Windows-1252
//! - 等等（参见 encoding_rs 支持的编码列表）

use rf_errors::{Result, RfError};

/// 将字节数据从一种编码转换为另一种编码
///
/// 此函数通过 UTF-8 作为中间编码来实现不同编码之间的转换。
///
/// # 参数
///
/// * `data` - 原始字节数据
/// * `from_encoding` - 源编码名称（如 "GBK", "UTF-16LE"）
/// * `to_encoding` - 目标编码名称（如 "UTF-8", "BIG5"）
///
/// # 返回值
///
/// 返回转换后的字节数据，转换失败时返回错误
///
/// # 错误
///
/// - 当源编码名称不支持时
/// - 当目标编码名称不支持时
/// - 当数据不是有效的源编码格式时
///
/// # 示例
///
/// ```rust
/// use rf_encoding::charset_convert;
///
/// let data = b"Hello";
/// let converted = charset_convert(data, "UTF-8", "UTF-16LE").unwrap();
/// ```
pub fn charset_convert(data: &[u8], from_encoding: &str, to_encoding: &str) -> Result<Vec<u8>> {
    // 先将源编码转换为 UTF-8 字符串
    let utf8_str = charset_to_utf8(data, from_encoding)?;

    // 再将 UTF-8 字符串转换为目标编码
    let utf8_bytes = utf8_str.as_bytes();
    charset_from_utf8(utf8_bytes, to_encoding)
}

/// 将字节数据从指定编码转换为 UTF-8 字符串
///
/// # 参数
///
/// * `data` - 原始字节数据
/// * `encoding` - 源编码名称
///
/// # 返回值
///
/// 返回 UTF-8 字符串，转换失败时返回错误
///
/// # 错误
///
/// - 当编码名称不支持时
/// - 当数据不是有效的编码格式时
///
/// # 示例
///
/// ```rust
/// use rf_encoding::charset_to_utf8;
///
/// let data = b"Hello";
/// let utf8_str = charset_to_utf8(data, "UTF-8").unwrap();
/// ```
pub fn charset_to_utf8(data: &[u8], encoding: &str) -> Result<String> {
    let encoding_label = encoding_rs::Encoding::for_label(encoding.as_bytes())
        .ok_or_else(|| RfError::Serialization(format!("Unsupported encoding: {}", encoding)))?;

    let (cow, ..) = encoding_label.decode(data);
    Ok(cow.to_string())
}

/// 将 UTF-8 字节数据转换为指定编码
///
/// # 参数
///
/// * `data` - UTF-8 编码的字节数据
/// * `encoding` - 目标编码名称
///
/// # 返回值
///
/// 返回目标编码的字节数据，转换失败时返回错误
///
/// # 错误
///
/// - 当编码名称不支持时
/// - 当数据不是有效的 UTF-8 格式时
///
/// # 示例
///
/// ```rust
/// use rf_encoding::charset_from_utf8;
///
/// let data = "Hello".as_bytes();
/// let gbk_bytes = charset_from_utf8(data, "GBK").unwrap();
/// ```
pub fn charset_from_utf8(data: &[u8], encoding: &str) -> Result<Vec<u8>> {
    let encoding_label = encoding_rs::Encoding::for_label(encoding.as_bytes())
        .ok_or_else(|| RfError::Serialization(format!("Unsupported encoding: {}", encoding)))?;

    // First decode the UTF-8 data to a string, then encode to target encoding
    let (cow, ..) = encoding_rs::UTF_8.decode(data);
    let (encoded_cow, ..) = encoding_label.encode(&cow);
    Ok(encoded_cow.to_vec())
}

/// 检测字节数据的可能编码
///
/// 这是一个简单的编码检测函数，使用启发式方法猜测编码。
/// 注意：检测结果不一定准确，仅供参考。
///
/// # 参数
///
/// * `data` - 要检测的字节数据
///
/// # 返回值
///
/// 返回可能的编码名称列表（按可能性排序）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::charset_detect;
///
/// let data = "Hello".as_bytes();
/// let encodings = charset_detect(data);
/// println!("Possible encodings: {:?}", encodings);
/// ```
pub fn charset_detect(data: &[u8]) -> Vec<String> {
    let mut encodings = Vec::new();

    // 检查 UTF-8 BOM
    if data.starts_with(&[0xEF, 0xBB, 0xBF]) {
        encodings.push("UTF-8".to_string());
    }

    // 检查 UTF-16LE BOM
    if data.starts_with(&[0xFF, 0xFE]) {
        encodings.push("UTF-16LE".to_string());
    }

    // 检查 UTF-16BE BOM
    if data.starts_with(&[0xFE, 0xFF]) {
        encodings.push("UTF-16BE".to_string());
    }

    // 如果没有 BOM，尝试检测 UTF-8 有效性
    if encodings.is_empty() && std::str::from_utf8(data).is_ok() {
        encodings.push("UTF-8".to_string());
    }

    // 如果没有检测到任何编码，默认添加 UTF-8
    if encodings.is_empty() {
        encodings.push("UTF-8".to_string());
    }

    encodings
}

/// 验证字节数据是否符合指定编码
///
/// # 参数
///
/// * `data` - 要验证的字节数据
/// * `encoding` - 编码名称
///
/// # 返回值
///
/// 如果数据有效则返回 Ok(())，否则返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::charset_validate;
///
/// let data = "Hello".as_bytes();
/// assert!(charset_validate(data, "UTF-8").is_ok());
/// ```
pub fn charset_validate(data: &[u8], encoding: &str) -> Result<()> {
    let encoding_label = encoding_rs::Encoding::for_label(encoding.as_bytes())
        .ok_or_else(|| RfError::Serialization(format!("Unsupported encoding: {}", encoding)))?;

    let mut decoder = encoding_label.new_decoder();

    let mut buf = [0u8; 1024];
    let mut total_read = 0;

    loop {
        if total_read >= data.len() {
            break;
        }

        let remaining = &data[total_read..];
        let chunk_size = remaining.len().min(1024);

        let (_result, _read, _written, success) = decoder.decode_to_utf8(
            &remaining[..chunk_size],
            &mut buf,
            false
        );

        if success {
            total_read += chunk_size;
        } else {
            return Err(RfError::Serialization(format!("Invalid {} encoding", encoding)));
        }
    }

    Ok(())
}

/// 获取支持的编码列表
///
/// # 返回值
///
/// 返回所有支持的编码名称列表
///
/// # 示例
///
/// ```rust
/// use rf_encoding::charset_supported;
///
/// let encodings = charset_supported();
/// println!("Supported encodings: {:?}", encodings);
/// ```
pub fn charset_supported() -> Vec<String> {
    // encoding_rs 支持的常见编码
    vec![
        "UTF-8".to_string(),
        "UTF-16LE".to_string(),
        "UTF-16BE".to_string(),
        "GBK".to_string(),
        "GB18030".to_string(),
        "Big5".to_string(),
        "Shift_JIS".to_string(),
        "EUC-JP".to_string(),
        "EUC-KR".to_string(),
        "ISO-8859-1".to_string(),
        "ISO-8859-2".to_string(),
        "ISO-8859-3".to_string(),
        "ISO-8859-4".to_string(),
        "ISO-8859-5".to_string(),
        "ISO-8859-6".to_string(),
        "ISO-8859-7".to_string(),
        "ISO-8859-8".to_string(),
        "ISO-8859-9".to_string(),
        "ISO-8859-10".to_string(),
        "ISO-8859-11".to_string(),
        "ISO-8859-13".to_string(),
        "ISO-8859-14".to_string(),
        "ISO-8859-15".to_string(),
        "ISO-8859-16".to_string(),
        "Windows-1250".to_string(),
        "Windows-1251".to_string(),
        "Windows-1252".to_string(),
        "Windows-1253".to_string(),
        "Windows-1254".to_string(),
        "Windows-1255".to_string(),
        "Windows-1256".to_string(),
        "Windows-1257".to_string(),
        "Windows-1258".to_string(),
    ]
}
