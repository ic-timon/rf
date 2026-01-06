//! # yaml
//!
//! yaml 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06
//! # YAML 编码/解码模块
//!
//! 提供 YAML（YAML Ain't Markup Language）格式的序列化和反序列化功能。
//! YAML 是一种人类可读的数据序列化格式，常用于配置文件。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{yaml_encode, yaml_decode, yaml_parse};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     name: String,
//!     value: i32,
//! }
//!
//! let config = Config { name: "test".to_string(), value: 42 };
//!
//! // 编码为 YAML
//! let yaml_str = yaml_encode(&config).unwrap();
//!
//! // 从 YAML 解码
//! let decoded: Config = yaml_decode(&yaml_str).unwrap();
//!
//! // 解析为动态 Value
//! let value = yaml_parse("name: test\nvalue: 42").unwrap();
//! ```

use rf_errors::{Result, RfError};
use serde::{Deserialize, Serialize};

/// 将实现了 `Serialize` trait 的值编码为 YAML 字符串
///
/// # 参数
///
/// * `value` - 要序列化的值引用，必须实现了 `Serialize` trait
///
/// # 返回值
///
/// 返回 YAML 字符串，序列化失败时返回错误
///
/// # 错误
///
/// 当值包含无法序列化的类型或数据结构不合法时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::yaml_encode;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Data {
///     name: String,
///     value: i32,
/// }
///
/// let data = Data { name: "test".to_string(), value: 42 };
/// let yaml = yaml_encode(&data).unwrap();
/// ```
pub fn encode<T: Serialize>(value: &T) -> Result<String> {
    serde_yaml::to_string(value)
        .map_err(|e| RfError::Serialization(format!("YAML encode error: {}", e)))
}

/// 将实现了 `Serialize` trait 的值编码为格式化的 YAML 字符串
///
/// 与 `encode` 不同，此函数会添加更好的格式化。
///
/// # 参数
///
/// * `value` - 要序列化的值引用，必须实现了 `Serialize` trait
///
/// # 返回值
///
/// 返回格式化的 YAML 字符串，序列化失败时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::yaml_encode_pretty;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Data {
///     name: String,
///     value: i32,
/// }
///
/// let data = Data { name: "test".to_string(), value: 42 };
/// let yaml = yaml_encode_pretty(&data).unwrap();
/// ```
pub fn encode_pretty<T: Serialize>(value: &T) -> Result<String> {
    serde_yaml::to_string(value)
        .map_err(|e| RfError::Serialization(format!("YAML encode error: {}", e)))
}

/// 从 YAML 字符串解码为指定的类型
///
/// # 类型参数
///
/// * `T` - 目标类型，必须实现了 `Deserialize` trait
///
/// # 参数
///
/// * `s` - YAML 字符串
///
/// # 返回值
///
/// 返回反序列化后的值，解析失败时返回错误
///
/// # 错误
///
/// 当 YAML 格式不正确、数据类型不匹配或缺少必需字段时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::yaml_decode;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Data {
///     name: String,
///     value: i32,
/// }
///
/// let yaml = "name: test\nvalue: 42";
/// let data: Data = yaml_decode(yaml).unwrap();
/// ```
pub fn decode<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T> {
    serde_yaml::from_str(s)
        .map_err(|e| RfError::Serialization(format!("YAML decode error: {}", e)))
}

/// 将 YAML 字符串解析为动态的 `serde_yaml::Value`
///
/// 与 `decode` 不同，此函数不需要指定具体类型，而是返回一个可以动态访问的值。
///
/// # 参数
///
/// * `s` - YAML 字符串
///
/// # 返回值
///
/// 返回 `serde_yaml::Value`，可以动态访问其内容，解析失败时返回错误
///
/// # 使用场景
///
/// - 当 YAML 结构不确定或经常变化时
/// - 当只需要提取部分字段时
/// - 当需要动态处理 YAML 数据时
///
/// # 示例
///
/// ```rust
/// use rf_encoding::yaml_parse;
///
/// let yaml = "name: 张三\nage: 25";
/// let value = yaml_parse(yaml).unwrap();
///
/// // 动态访问字段
/// assert_eq!(value["name"].as_str(), Some("张三"));
/// assert_eq!(value["age"].as_i64(), Some(25));
/// ```
pub fn parse(s: &str) -> Result<serde_yaml::Value> {
    serde_yaml::from_str(s)
        .map_err(|e| RfError::Serialization(format!("YAML parse error: {}", e)))
}
