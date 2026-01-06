//! # json
//!
//! json 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # JSON 编码/解码模块
//!
//! 提供 JSON（JavaScript Object Notation）格式的序列化和反序列化功能。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{json_encode, json_encode_pretty, json_decode, json_parse};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! let person = Person { name: "张三".to_string(), age: 25 };
//!
//! // 普通编码
//! let json_str = json_encode(&person).unwrap();
//!
//! // 格式化编码（带缩进）
//! let pretty_str = json_encode_pretty(&person).unwrap();
//!
//! // 解码
//! let decoded: Person = json_decode(&json_str).unwrap();
//!
//! // 解析为动态 Value
//! let value = json_parse(r#"{"name": "李四", "age": 30}"#).unwrap();
//! ```

use rf_errors::{Result, RfError};
use serde::{Deserialize, Serialize};

/// 将实现了 `Serialize` trait 的值编码为 JSON 字符串
///
/// # 参数
///
/// * `value` - 要序列化的值引用，必须实现了 `Serialize` trait
///
/// # 返回值
///
/// 返回 JSON 字符串，序列化失败时返回错误
///
/// # 错误
///
/// 当值包含无法序列化的类型（如互斥引用）或数据结构不合法时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::json_encode;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Data {
///     id: u32,
///     name: String,
/// }
///
/// let data = Data { id: 1, name: "测试".to_string() };
/// let json = json_encode(&data).unwrap();
/// assert_eq!(json, r#"{"id":1,"name":"测试"}"#);
/// ```
pub fn encode<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value)
        .map_err(|e| RfError::Serialization(format!("JSON encode error: {}", e)))
}

/// 将实现了 `Serialize` trait 的值编码为格式化的 JSON 字符串
///
/// 与 `encode` 不同，此函数会添加缩进和换行，使输出更易读。
///
/// # 参数
///
/// * `value` - 要序列化的值引用，必须实现了 `Serialize` trait
///
/// # 返回值
///
/// 返回格式化的 JSON 字符串（带缩进和换行），序列化失败时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::json_encode_pretty;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Data {
///     id: u32,
///     name: String,
/// }
///
/// let data = Data { id: 1, name: "测试".to_string() };
/// let json = json_encode_pretty(&data).unwrap();
/// // 输出会包含缩进和换行
/// ```
pub fn encode_pretty<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string_pretty(value)
        .map_err(|e| RfError::Serialization(format!("JSON encode error: {}", e)))
}

/// 从 JSON 字符串解码为指定的类型
///
/// # 类型参数
///
/// * `T` - 目标类型，必须实现了 `Deserialize` trait
///
/// # 参数
///
/// * `s` - JSON 字符串
///
/// # 返回值
///
/// 返回反序列化后的值，解析失败时返回错误
///
/// # 错误
///
/// 当 JSON 格式不正确、数据类型不匹配或缺少必需字段时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::json_decode;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Data {
///     id: u32,
///     name: String,
/// }
///
/// let json = r#"{"id":1,"name":"测试"}"#;
/// let data: Data = json_decode(json).unwrap();
/// assert_eq!(data.id, 1);
/// assert_eq!(data.name, "测试");
/// ```
pub fn decode<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T> {
    serde_json::from_str(s)
        .map_err(|e| RfError::Serialization(format!("JSON decode error: {}", e)))
}

/// 将 JSON 字符串解析为动态的 `serde_json::Value`
///
/// 与 `decode` 不同，此函数不需要指定具体类型，而是返回一个可以动态访问的值。
///
/// # 参数
///
/// * `s` - JSON 字符串
///
/// # 返回值
///
/// 返回 `serde_json::Value`，可以动态访问其内容，解析失败时返回错误
///
/// # 使用场景
///
/// - 当 JSON 结构不确定或经常变化时
/// - 当只需要提取部分字段时
/// - 当需要动态处理 JSON 数据时
///
/// # 示例
///
/// ```rust
/// use rf_encoding::json_parse;
///
/// let json = r#"{"name": "张三", "age": 25, "city": "北京"}"#;
/// let value = json_parse(json).unwrap();
///
/// // 动态访问字段
/// assert_eq!(value["name"], "张三");
/// assert_eq!(value["age"], 25);
/// ```
pub fn parse(s: &str) -> Result<serde_json::Value> {
    serde_json::from_str(s)
        .map_err(|e| RfError::Serialization(format!("JSON parse error: {}", e)))
}

