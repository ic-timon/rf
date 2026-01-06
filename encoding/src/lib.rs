//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # RF 编码模块
//!
//! 提供各种编码和解码功能，支持多种数据格式和编码方式。
//!
//! ## 支持的格式
//!
//! - **数据序列化格式**: JSON, YAML, TOML, XML
//! - **配置文件格式**: INI, Properties
//! - **编码格式**: Base64, Binary, Charset
//! - **压缩格式**: Gzip, Zlib
//! - **哈希算法**: XXHash, 通用 Hash
//! - **Web 格式**: HTML, URL
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{json_encode, json_decode, base64_encode, base64_decode};
//!
//! // JSON 编码和解码
//! let data = serde_json::json!({"name": "张三", "age": 25});
//! let json_str = json_encode(&data).unwrap();
//! let decoded: serde_json::Value = json_decode(&json_str).unwrap();
//!
//! // Base64 编码和解码
//! let encoded = base64_encode(b"Hello World");
//! let decoded = base64_decode(&encoded).unwrap();
//! ```
//!
//! @author TimonQWQ
//! @date 2026-01-06

pub mod json;
pub mod yaml;
pub mod toml;
pub mod xml;
pub mod ini;
pub mod properties;
pub mod base64;
pub mod binary;
pub mod charset;
pub mod compress;
pub mod hash;
pub mod html;
pub mod url;

// 重新导出各个模块的函数，避免命名冲突
pub use json::{encode as json_encode, encode_pretty as json_encode_pretty, decode as json_decode, parse as json_parse};
pub use yaml::{encode as yaml_encode, decode as yaml_decode, parse as yaml_parse};
pub use toml::{encode as toml_encode, encode_pretty as toml_encode_pretty, decode as toml_decode};
pub use xml::{encode as xml_encode, decode as xml_decode};
pub use ini::{parse as ini_parse, encode as ini_encode};
pub use properties::{parse as properties_parse, encode as properties_encode};
pub use base64::{encode as base64_encode, decode as base64_decode};
pub use binary::*;
pub use charset::*;
pub use compress::*;
pub use hash::*;
// 使用特定导入避免名称冲突
pub use html::{parse as html_parse, extract_text, select};
pub use url::{parse as url_parse, scheme, host, path, query};
