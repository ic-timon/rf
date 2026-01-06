//! # compress
//!
//! compress 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06
//! # 数据压缩模块
//!
//! 提供 Gzip 和 Zlib 两种压缩算法的数据压缩和解压缩功能。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{
//!     gzip_compress, gzip_decompress,
//!     zlib_compress, zlib_decompress
//! };
//!
//! // Gzip 压缩和解压缩
//! let data = b"Hello, World! This is a test data.";
//! let compressed = gzip_compress(data).unwrap();
//! let decompressed = gzip_decompress(&compressed).unwrap();
//! assert_eq!(data, decompressed.as_slice());
//!
//! // Zlib 压缩和解压缩
//! let compressed = zlib_compress(data).unwrap();
//! let decompressed = zlib_decompress(&compressed).unwrap();
//! assert_eq!(data, decompressed.as_slice());
//! ```
//!
//! ## 压缩算法说明
//!
//! ### Gzip
//! - 广泛用于文件压缩（.gz 文件）
//! - 包含文件头和校验和
//! - HTTP 传输常用
//! - 压缩率略低于 Zlib
//!
//! ### Zlib
//! - 更紧凑的格式
//! - 常用于网络协议和数据传输
//! - PNG 图像格式使用 Zlib 压缩
//! - 压缩率略高于 Gzip

use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};
use flate2::Compression;
use rf_errors::{Result, RfError};
use std::io::{Read, Write};

/// 使用 Gzip 算法压缩数据
///
/// Gzip 是一种广泛使用的压缩格式，常用于文件压缩和 HTTP 传输。
///
/// # 参数
///
/// * `data` - 要压缩的字节数据
///
/// # 返回值
///
/// 返回压缩后的字节数据，压缩失败时返回错误
///
/// # 压缩级别
///
/// 使用默认压缩级别（通常是 6），在压缩率和速度之间取得平衡。
///
/// # 示例
///
/// ```rust
/// use rf_encoding::gzip_compress;
///
/// let data = b"Hello, World!";
/// let compressed = gzip_compress(data).unwrap();
/// println!("Original: {}, Compressed: {}", data.len(), compressed.len());
/// ```
pub fn gzip_compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).map_err(|e| RfError::Serialization(format!("Gzip compress error: {}", e)))?;
    encoder.finish().map_err(|e| RfError::Serialization(format!("Gzip compress finish error: {}", e)))
}

/// 使用 Gzip 算法压缩数据，指定压缩级别
///
/// # 参数
///
/// * `data` - 要压缩的字节数据
/// * `level` - 压缩级别（0-9），0 表示无压缩，9 表示最高压缩率
///
/// # 返回值
///
/// 返回压缩后的字节数据，压缩失败时返回错误
///
/// # 压缩级别说明
///
/// - 0: 无压缩
/// - 1-3: 快速压缩，压缩率较低
/// - 4-6: 平衡模式（推荐）
/// - 7-9: 最大压缩，速度较慢
///
/// # 示例
///
/// ```rust
/// use rf_encoding::gzip_compress_with_level;
///
/// let data = b"Hello, World!";
/// let compressed = gzip_compress_with_level(data, 9).unwrap(); // 最大压缩
/// ```
pub fn gzip_compress_with_level(data: &[u8], level: u8) -> Result<Vec<u8>> {
    let compression = Compression::new(level as u32);
    let mut encoder = GzEncoder::new(Vec::new(), compression);
    encoder.write_all(data).map_err(|e| RfError::Serialization(format!("Gzip compress error: {}", e)))?;
    encoder.finish().map_err(|e| RfError::Serialization(format!("Gzip compress finish error: {}", e)))
}

/// 使用 Gzip 算法解压缩数据
///
/// # 参数
///
/// * `data` - Gzip 压缩的字节数据
///
/// # 返回值
///
/// 返回解压缩后的原始数据，解压缩失败时返回错误
///
/// # 错误
///
/// 当数据不是有效的 Gzip 格式或数据损坏时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::{gzip_compress, gzip_decompress};
///
/// let original = b"Hello, World!";
/// let compressed = gzip_compress(original).unwrap();
/// let decompressed = gzip_decompress(&compressed).unwrap();
/// assert_eq!(original, decompressed.as_slice());
/// ```
pub fn gzip_decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result).map_err(|e| RfError::Serialization(format!("Gzip decompress error: {}", e)))?;
    Ok(result)
}

/// 使用 Zlib 算法压缩数据
///
/// Zlib 是一种更紧凑的压缩格式，常用于网络协议和数据传输。
///
/// # 参数
///
/// * `data` - 要压缩的字节数据
///
/// # 返回值
///
/// 返回压缩后的字节数据，压缩失败时返回错误
///
/// # 压缩级别
///
/// 使用默认压缩级别（通常是 6），在压缩率和速度之间取得平衡。
///
/// # 示例
///
/// ```rust
/// use rf_encoding::zlib_compress;
///
/// let data = b"Hello, World!";
/// let compressed = zlib_compress(data).unwrap();
/// println!("Original: {}, Compressed: {}", data.len(), compressed.len());
/// ```
pub fn zlib_compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).map_err(|e| RfError::Serialization(format!("Zlib compress error: {}", e)))?;
    encoder.finish().map_err(|e| RfError::Serialization(format!("Zlib compress finish error: {}", e)))
}

/// 使用 Zlib 算法压缩数据，指定压缩级别
///
/// # 参数
///
/// * `data` - 要压缩的字节数据
/// * `level` - 压缩级别（0-9），0 表示无压缩，9 表示最高压缩率
///
/// # 返回值
///
/// 返回压缩后的字节数据，压缩失败时返回错误
///
/// # 压缩级别说明
///
/// - 0: 无压缩
/// - 1-3: 快速压缩，压缩率较低
/// - 4-6: 平衡模式（推荐）
/// - 7-9: 最大压缩，速度较慢
///
/// # 示例
///
/// ```rust
/// use rf_encoding::zlib_compress_with_level;
///
/// let data = b"Hello, World!";
/// let compressed = zlib_compress_with_level(data, 9).unwrap(); // 最大压缩
/// ```
pub fn zlib_compress_with_level(data: &[u8], level: u8) -> Result<Vec<u8>> {
    let compression = Compression::new(level as u32);
    let mut encoder = ZlibEncoder::new(Vec::new(), compression);
    encoder.write_all(data).map_err(|e| RfError::Serialization(format!("Zlib compress error: {}", e)))?;
    encoder.finish().map_err(|e| RfError::Serialization(format!("Zlib compress finish error: {}", e)))
}

/// 使用 Zlib 算法解压缩数据
///
/// # 参数
///
/// * `data` - Zlib 压缩的字节数据
///
/// # 返回值
///
/// 返回解压缩后的原始数据，解压缩失败时返回错误
///
/// # 错误
///
/// 当数据不是有效的 Zlib 格式或数据损坏时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::{zlib_compress, zlib_decompress};
///
/// let original = b"Hello, World!";
/// let compressed = zlib_compress(original).unwrap();
/// let decompressed = zlib_decompress(&compressed).unwrap();
/// assert_eq!(original, decompressed.as_slice());
/// ```
pub fn zlib_decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result).map_err(|e| RfError::Serialization(format!("Zlib decompress error: {}", e)))?;
    Ok(result)
}
