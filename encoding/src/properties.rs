// # properties
//!
//! properties 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06
//! # Properties 配置文件编码/解码模块
//!
//! 提供 Java Properties 格式配置文件的解析和编码功能。
//! Properties 格式是一种简单的键值对配置文件格式，广泛用于 Java 应用程序。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{properties_parse, properties_encode};
// use std::collections::HashMap;
//!
//! let props_content = r#"
//! # 这是注释
//! app.name=我的应用
//! app.version=1.0.0
//! database.url=jdbc:mysql://localhost:3306/mydb
//! "#;
//!
//! // 解析 properties 文件
//! let props = properties_parse(props_content).unwrap();
//!
//! // 访问配置
//! let app_name = &props["app.name"];
//! let app_version = &props["app.version"];
//!
//! // 修改配置
//! let mut new_props = HashMap::new();
//! new_props.insert("key".to_string(), "value".to_string());
//!
//! // 编码为 properties 字符串
//! let props_string = properties_encode(&new_props);
//! ```
//!
//! ## Properties 格式说明
//!
//! - 简单的键值对格式，用 `=` 分隔
//! - 支持注释，以 `#` 开头
//! - 每行一个键值对
//! - 适合扁平化的配置，不支持嵌套结构

use rf_errors::Result;
use std::collections::HashMap;

/// Properties 配置类型
///
/// 表示 Properties 配置文件的数据结构。
/// 使用 HashMap 存储键值对。
pub type PropertiesConfig = HashMap<String, String>;

/// 解析 Properties 格式的配置字符串
///
/// # 参数
///
/// * `content` - Properties 格式的配置字符串
///
/// # 返回值
///
/// 返回解析后的配置结构，解析失败时返回错误
///
/// # 错误
///
/// - 当键值对格式不正确时
/// - 当转义字符无效时
///
/// # 示例
///
/// ```rust
/// use rf_encoding::properties_parse;
///
/// let props = "key=value";
/// let config = properties_parse(props).unwrap();
/// assert_eq!(config["key"], "value");
/// ```
pub fn parse(content: &str) -> Result<PropertiesConfig> {
    let mut config = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        // 跳过空行和注释
        if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
            continue;
        }

        // 解析键值对
        let (key, value) = if let Some(sep_pos) = line.find('=') {
            (&line[..sep_pos], &line[sep_pos + 1..])
        } else if let Some(colon_pos) = line.find(':') {
            (&line[..colon_pos], &line[colon_pos + 1..])
        } else if let Some(space_pos) = line.find(' ') {
            (&line[..space_pos], &line[space_pos + 1..])
        } else {
            continue;
        };

        let key = key.trim().to_string();
        let value = value.trim().to_string();

        if !key.is_empty() {
            config.insert(key, value);
        }
    }

    Ok(config)
}

/// 将配置结构编码为 Properties 格式字符串
///
/// # 参数
///
/// * `config` - 配置结构
///
/// # 返回值
///
/// 返回 Properties 格式的字符串
///
/// # 示例
///
/// ```rust
/// use rf_encoding::properties_encode;
/// use std::collections::HashMap;
///
/// let mut config = HashMap::new();
/// config.insert("key".to_string(), "value".to_string());
///
/// let props = properties_encode(&config);
/// ```
pub fn encode(config: &PropertiesConfig) -> String {
    config
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("\n")
}

/// 从配置结构中获取指定键的值
///
/// # 参数
///
/// * `config` - 配置结构
/// * `key` - 键名
///
/// # 返回值
///
/// 返回值的 Option，如果不存在则返回 None
///
/// # 示例
///
/// ```rust
/// use rf_encoding::{properties_parse, properties_get};
///
/// let props = "key=value";
/// let config = properties_parse(props).unwrap();
/// assert_eq!(properties_get(&config, "key"), Some(&"value".to_string()));
/// ```
pub fn get<'a>(config: &'a PropertiesConfig, key: &str) -> Option<&'a String> {
    config.get(key)
}

/// 设置配置结构中指定键的值
///
/// # 参数
///
/// * `config` - 配置结构（可变引用）
/// * `key` - 键名
/// * `value` - 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::properties_set;
/// use std::collections::HashMap;
///
/// let mut config = HashMap::new();
/// properties_set(&mut config, "key", "value");
/// ```
pub fn set(config: &mut PropertiesConfig, key: &str, value: &str) {
    config.insert(key.to_string(), value.to_string());
}

/// 加载 Properties 文件
///
/// # 参数
///
/// * `path` - 文件路径
///
/// # 返回值
///
/// 返回解析后的配置结构，读取或解析失败时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::properties_load;
///
/// let config = properties_load("config.properties").unwrap();
/// ```
pub fn load(path: impl AsRef<std::path::Path>) -> Result<PropertiesConfig> {
    let content = std::fs::read_to_string(path)?;
    parse(&content)
}

/// 保存配置到 Properties 文件
///
/// # 参数
///
/// * `config` - 配置结构
/// * `path` - 文件路径
///
/// # 返回值
///
/// 保存失败时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::properties_save;
/// use std::collections::HashMap;
///
/// let mut config = HashMap::new();
/// properties_save(&config, "config.properties").unwrap();
/// ```
pub fn save(config: &PropertiesConfig, path: impl AsRef<std::path::Path>) -> Result<()> {
    let content = encode(config);
    std::fs::write(path, content)?;
    Ok(())
}
