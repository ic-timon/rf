// # ini
//!
//! ini 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06
//! # INI 配置文件编码/解码模块
//!
//! 提供 INI 格式配置文件的解析和编码功能。
//! INI 格式是一种简单的配置文件格式，广泛用于 Windows 系统配置。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{ini_parse, ini_encode};
//! use std::collections::HashMap;
//!
//! let ini_content = r#"
//! [database]
//! host = localhost
//! port = 5432
//!
//! [server]
//! host = 0.0.0.0
//! port = 8080
//! "#;
//!
//! // 解析 INI 文件
//! let config = ini_parse(ini_content).unwrap();
//!
//! // 访问配置
//! let db_host = &config["database"]["host"];
//! let db_port = &config["database"]["port"];
//!
//! // 修改配置
//! let mut new_config = HashMap::new();
//! let mut section = HashMap::new();
//! section.insert("key".to_string(), "value".to_string());
//! new_config.insert("section".to_string(), section);
//!
//! // 编码为 INI 字符串
//! let ini_string = ini_encode(&new_config);
//! ```
//!
//! ## INI 格式说明
//!
//! - 由节（section）组成，节名用方括号 `[section]` 包围
//! - 每个节包含多个键值对，用 `=` 分隔
//! - 支持注释，以 `;` 或 `#` 开头
//! - 简单直观，适合层级不深的配置

use rf_errors::Result;
use std::collections::HashMap;

/// INI 配置类型
///
/// 表示 INI 配置文件的数据结构。
/// 外层 HashMap 的键是节名，内层 HashMap 的键是配置项名，值是配置项的值。
pub type IniConfig = HashMap<String, HashMap<String, String>>;

/// 解析 INI 格式的配置字符串
///
/// # 参数
///
/// * `content` - INI 格式的配置字符串
///
/// # 返回值
///
/// 返回解析后的配置结构，解析失败时返回错误
///
/// # 错误
///
/// - 当 INI 格式不正确时
// - 当节名格式无效时
///
/// # 示例
///
/// ```rust
/// use rf_encoding::ini_parse;
///
/// let ini = "[section]\nkey = value";
/// let config = ini_parse(ini).unwrap();
/// assert_eq!(config["section"]["key"], "value");
/// ```
pub fn parse(content: &str) -> Result<IniConfig> {
    let mut config = IniConfig::new();
    let mut current_section: String = String::new();

    for line in content.lines() {
        let line = line.trim();

        // 跳过空行和注释
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        // 解析节
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].trim().to_string();
            if !config.contains_key(&current_section) {
                config.insert(current_section.clone(), HashMap::new());
            }
            continue;
        }

        // 解析键值对
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().to_string();

            if current_section.is_empty() {
                // 没有节的情况，使用空字符串作为节名
                if !config.contains_key(&current_section) {
                    config.insert(current_section.clone(), HashMap::new());
                }
            }

            if let Some(section) = config.get_mut(&current_section) {
                section.insert(key, value);
            }
        }
    }

    Ok(config)
}

/// 将配置结构编码为 INI 格式字符串
///
/// # 参数
///
/// * `config` - 配置结构
///
/// # 返回值
///
/// 返回 INI 格式的字符串
///
/// # 示例
///
/// ```rust
/// use rf_encoding::ini_encode;
/// use std::collections::HashMap;
///
/// let mut config = HashMap::new();
/// let mut section = HashMap::new();
/// section.insert("key".to_string(), "value".to_string());
/// config.insert("section".to_string(), section);
///
/// let ini = ini_encode(&config);
/// ```
pub fn encode(config: &IniConfig) -> String {
    let mut result = String::new();

    for (section_name, section) in config {
        if !section_name.is_empty() {
            result.push_str(&format!("[{}]\n", section_name));
        }

        for (key, value) in section {
            result.push_str(&format!("{} = {}\n", key, value));
        }

        result.push('\n');
    }

    result
}

/// 从配置结构中获取指定节的指定键的值
///
/// # 参数
///
/// * `config` - 配置结构
/// * `section` - 节名
/// * `key` - 键名
///
/// # 返回值
///
/// 返回值的 Option，如果不存在则返回 None
///
/// # 示例
///
/// ```rust
/// use rf_encoding::{ini_parse, ini_get};
///
/// let ini = "[section]\nkey = value";
/// let config = ini_parse(ini).unwrap();
/// assert_eq!(ini_get(&config, "section", "key"), Some(&"value".to_string()));
/// ```
pub fn get<'a>(config: &'a IniConfig, section: &str, key: &str) -> Option<&'a String> {
    config.get(section)?.get(key)
}

/// 设置配置结构中指定节的指定键的值
///
/// # 参数
///
/// * `config` - 配置结构（可变引用）
/// * `section` - 节名
/// * `key` - 键名
/// * `value` - 值
///
/// # 示例
///
/// ```rust
/// use rf_encoding::ini_set;
/// use std::collections::HashMap;
///
/// let mut config = HashMap::new();
/// ini_set(&mut config, "section", "key", "value");
/// ```
pub fn set(config: &mut IniConfig, section: &str, key: &str, value: &str) {
    if !config.contains_key(section) {
        config.insert(section.to_string(), HashMap::new());
    }
    if let Some(sec) = config.get_mut(section) {
        sec.insert(key.to_string(), value.to_string());
    }
}
