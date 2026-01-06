//! # meta
//!
//! meta 模块 - 元数据管理工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 元数据管理工具模块
//!
//! 本模块提供了元数据的存储和管理功能，包括：
//! - Meta trait：用于标记可以提供元数据的类型
//! - Metadata 结构体：通用的键值对元数据容器

use serde::{Deserialize, Serialize};

/// 元数据 trait
///
/// 实现此 trait 的类型可以提供自己的元数据信息。
/// 元数据通常用于存储对象的附加信息，如创建时间、修改时间、标签等。
///
/// # 示例
/// ```ignore
/// use rf_util::meta::Meta;
/// use serde_json::json;
///
/// struct MyStruct {
///     name: String,
/// }
///
/// impl Meta for MyStruct {
///     fn meta(&self) -> serde_json::Value {
///         json!({
///             "name": self.name,
///             "type": "MyStruct"
///         })
///     }
/// }
/// ```
pub trait Meta {
    /// 获取元数据
    ///
    /// # 返回值
    /// 返回一个 JSON Value，包含该对象的所有元数据信息
    fn meta(&self) -> serde_json::Value;
}

/// 元数据包装器
///
/// 一个通用的键值对容器，用于存储和管理任意 JSON 格式的元数据。
///
/// # 字段说明
/// - `data`: 存储元数据的 HashMap，键为字符串，值为 JSON Value
///
/// # 示例
/// ```ignore
/// use rf_util::meta::Metadata;
/// use serde_json::json;
///
/// let mut metadata = Metadata::new();
/// metadata.set("created_at", json!("2024-01-01"));
/// metadata.set("author", json!("TimonQWQ"));
///
/// if let Some(created_at) = metadata.get("created_at") {
///     println!("Created at: {}", created_at);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub data: serde_json::Map<String, serde_json::Value>,
}

impl Metadata {
    /// 创建新的元数据容器
    ///
    /// # 返回值
    /// 返回一个空的 Metadata 实例
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::meta::Metadata;
    ///
    /// let metadata = Metadata::new();
    /// ```
    pub fn new() -> Self {
        Self {
            data: serde_json::Map::new(),
        }
    }

    /// 设置元数据值
    ///
    /// # 参数
    /// - `key`: 元数据的键
    /// - `value`: 元数据的值，可以是任意 JSON 兼容的值
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::meta::Metadata;
    /// use serde_json::json;
    ///
    /// let mut metadata = Metadata::new();
    /// metadata.set("version", json!(1));
    /// metadata.set("name", json!("my_app"));
    /// ```
    pub fn set(&mut self, key: &str, value: serde_json::Value) {
        self.data.insert(key.to_string(), value);
    }

    /// 获取元数据值
    ///
    /// # 参数
    /// - `key`: 要获取的元数据的键
    ///
    /// # 返回值
    /// - `Some(&serde_json::Value)`: 如果键存在，返回对应的值
    /// - `None`: 如果键不存在
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::meta::Metadata;
    /// use serde_json::json;
    ///
    /// let mut metadata = Metadata::new();
    /// metadata.set("key1", json!("value1"));
    ///
    /// assert_eq!(metadata.get("key1"), Some(&json!("value1")));
    /// assert_eq!(metadata.get("nonexistent"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.data.get(key)
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

