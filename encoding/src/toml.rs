//! # toml
//!
//! toml 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06
//! # TOML 编码/解码模块
//!
//! 提供 TOML（Tom's Obvious, Minimal Language）格式的序列化和反序列化功能。
//! TOML 是一种简洁的配置文件格式，被广泛应用于 Rust 项目的配置管理。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{toml_encode, toml_encode_pretty, toml_decode};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     name: String,
//!     version: String,
//!     debug: bool,
//!     port: u16,
//! }
//!
//! let config = Config {
//!     name: "myapp".to_string(),
//!     version: "1.0.0".to_string(),
//!     debug: true,
//!     port: 8080,
//! };
//!
//! // 编码为 TOML
//! let toml_str = toml_encode(&config).unwrap();
//!
//! // 编码为格式化的 TOML
//! let pretty = toml_encode_pretty(&config).unwrap();
//!
//! // 从 TOML 解码
//! let decoded: Config = toml_decode(&toml_str).unwrap();
//! ```
//!
//! ## TOML 格式特点
//!
//! - 语法简单直观
//! - 支持注释（以 # 开头）
//! - 明确的数据类型
//! - 适合配置文件
//! - 广泛应用于 Rust 生态系统（如 Cargo.toml）

use rf_errors::{Result, RfError};
use serde::{Deserialize, Serialize};

/// 将实现了 `Serialize` trait 的值编码为 TOML 字符串
///
/// # 参数
///
/// * `value` - 要序列化的值引用，必须实现了 `Serialize` trait
///
/// # 返回值
///
/// 返回 TOML 字符串，序列化失败时返回错误
///
/// # 错误
///
/// 当值包含无法序列化的类型或数据结构不合法时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::toml_encode;
/// use serde::Serialize;
///
/// // #[derive(Serialize)]
/// struct Config {
/// //     name: String,
/// //     port: u16,
/// // }
///
/// let config = Config { name: "myapp".to_string(), port: 8080 };
/// let toml = toml_encode(&config).unwrap();
/// ```
pub fn encode<T: Serialize>(value: &T) -> Result<String> {
    toml::to_string(value)
        .map_err(|e| RfError::Serialization(format!("TOML encode error: {}", e)))
}

/// 将实现了 `Serialize` trait 的值编码为格式化的 TOML 字符串
///
/// 与 `encode` 不同，此函数会添加更好的格式化。
///
/// # 参数
///
/// * `value` - 要序列化的值引用，必须实现了 `Serialize` trait
///
/// # 返回值
///
/// 返回格式化的 TOML 字符串，序列化失败时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::toml_encode_pretty;
/// use serde::Serialize;
///
/// // #[derive(Serialize)]
/// struct Config {
/// //     name: String,
/// //     port: u16,
/// // }
///
/// let config = Config { name: "myapp".to_string(), port: 8080 };
/// let toml = toml_encode_pretty(&config).unwrap();
/// ```
pub fn encode_pretty<T: Serialize>(value: &T) -> Result<String> {
    toml::to_string_pretty(value)
        .map_err(|e| RfError::Serialization(format!("TOML encode error: {}", e)))
}

/// 从 TOML 字符串解码为指定的类型
///
/// # 类型参数
///
/// * `T` - 目标类型，必须实现了 `Deserialize` trait
///
/// # 参数
///
/// * `s` - TOML 字符串
///
/// # 返回值
///
/// 返回反序列化后的值，解析失败时返回错误
///
/// # 错误
///
/// 当 TOML 格式不正确、数据类型不匹配或缺少必需字段时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::toml_decode;
/// use serde::Deserialize;
///
/// // #[derive(Deserialize)]
/// struct Config {
/// //     name: String,
/// //     port: u16,
/// // }
///
/// let toml = "name = \"myapp\"\nport = 8080";
/// let config: Config = toml_decode(toml).unwrap();
/// ```
pub fn decode<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T> {
    toml::from_str(s)
        .map_err(|e| RfError::Serialization(format!("TOML decode error: {}", e)))
}
