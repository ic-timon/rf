//! # tag
//!
//! tag 模块 - 标签管理工具
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 标签管理工具模块
//!
//! 本模块提供了标签的存储和管理功能，用于给对象添加、查询和删除标签。
//! 常见于资源分类、权限控制、对象标记等场景。

use std::collections::HashMap;

/// 标签管理器
///
/// 用于管理对象的标签集合，支持标签的增删查操作。
/// 标签以键值对的形式存储，键和值都是字符串类型。
///
/// # 字段说明
/// - `tags`: 存储标签的 HashMap，键为标签名，值为标签内容
///
/// # 示例
/// ```ignore
/// use rf_util::tag::TagManager;
///
/// let mut manager = TagManager::new();
/// manager.set("environment", "production");
/// manager.set("version", "1.0.0");
///
/// assert_eq!(manager.get("environment"), Some(&"production".to_string()));
/// ```
#[derive(Debug, Clone)]
pub struct TagManager {
    tags: HashMap<String, String>,
}

impl TagManager {
    /// 创建新的标签管理器
    ///
    /// # 返回值
    /// 返回一个空的 TagManager 实例
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::tag::TagManager;
    ///
    /// let manager = TagManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
        }
    }

    /// 设置标签
    ///
    /// 如果标签已存在，将更新其值；如果不存在，将创建新标签。
    ///
    /// # 参数
    /// - `key`: 标签的键
    /// - `value`: 标签的值
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::tag::TagManager;
    ///
    /// let mut manager = TagManager::new();
    /// manager.set("color", "red");
    /// manager.set("size", "large");
    /// ```
    pub fn set(&mut self, key: &str, value: &str) {
        self.tags.insert(key.to_string(), value.to_string());
    }

    /// 获取标签值
    ///
    /// # 参数
    /// - `key`: 要获取的标签的键
    ///
    /// # 返回值
    /// - `Some(&String)`: 如果标签存在，返回标签值的引用
    /// - `None`: 如果标签不存在
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::tag::TagManager;
    ///
    /// let mut manager = TagManager::new();
    /// manager.set("key1", "value1");
    ///
    /// assert_eq!(manager.get("key1"), Some(&"value1".to_string()));
    /// assert_eq!(manager.get("nonexistent"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&String> {
        self.tags.get(key)
    }

    /// 删除标签
    ///
    /// # 参数
    /// - `key`: 要删除的标签的键
    ///
    /// # 示例
    /// ```ignore
    /// use rf_util::tag::TagManager;
    ///
    /// let mut manager = TagManager::new();
    /// manager.set("temp", "value");
    /// manager.remove("temp");
    ///
    /// assert_eq!(manager.get("temp"), None);
    /// ```
    pub fn remove(&mut self, key: &str) {
        self.tags.remove(key);
    }
}

impl Default for TagManager {
    fn default() -> Self {
        Self::new()
    }
}

