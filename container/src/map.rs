//! # map 模块
//!
//! map 模块 - 映射容器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 映射容器
//!
//! 提供两种映射容器：
//! - `HashMap`：线程安全的哈希映射，基于 `DashMap` 实现
//! - `OrderedMap`：保持插入顺序的映射，基于 `IndexMap` 实现
//!
//! # 特性
//!
//! ## HashMap
//! - 线程安全：多个线程可以同时读写
//! - 高性能：使用分段锁实现高并发
//! - Clone 支持：可以克隆映射引用在多个线程间共享
//!
//! ## OrderedMap
//! - 插入顺序：保持键值对的插入顺序
//! - 快速查找：O(1) 平均时间复杂度的查找
//! - 有序迭代：按照插入顺序迭代键值对
//!
//! # 示例
//!
//! ## HashMap 线程安全示例
//!
//! ```
//! use rf_container::HashMap;
//! use std::thread;
//!
//! let map = HashMap::new();
//!
//! // 多个线程同时写入
//! for i in 0..10 {
//! //     let map_clone = map.clone();
//! //     thread::spawn(move || {
//! //         map_clone.insert(i, i * 2);
//! //     });
//! // }
//! ```
//!
//! ## OrderedMap 示例
//!
//! ```
//! use rf_container::OrderedMap;
//!
//! let mut map = OrderedMap::new();
//! map.insert("a", 1);
//! map.insert("b", 2);
//! map.insert("c", 3);
//!
//! // 按照插入顺序遍历
//! for (key, value) in &map {
//!     println!("{}: {}", key, value);
//! // }
//! ```

use dashmap::DashMap;
use indexmap::IndexMap;
use serde_json::Value;
use std::hash::Hash;

/// 线程安全的 HashMap 包装器
///
/// 封装了 `DashMap`，提供线程安全的键值存储。
/// DashMap 使用分段锁技术，在多线程环境下比 `Mutex<HashMap>` 性能更好。
///
/// # 字段
///
/// - `0`: 内部的 `DashMap<K, V>`，存储键值对
///
/// # 类型参数
///
/// * `K`: 键的类型，必须实现 `Hash` 和 `Eq` trait
/// * `V`: 值的类型
///
/// # 线程安全
///
/// `HashMap` 是完全线程安全的，多个线程可以同时调用任意方法。
///
/// # 示例
///
/// ```
/// use rf_container::HashMap;
///
/// let map = HashMap::new();
/// map.insert("key1", "value1");
/// map.insert("key2", "value2");
///
/// assert_eq!(map.get(&"key1").map(|v| *v), Some("value1"));
/// ```
#[derive(Debug)]
pub struct HashMap<K, V>(DashMap<K, V>)
where
    K: Hash + Eq;

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    /// 创建一个新的空 HashMap
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `HashMap` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::HashMap;
    ///
    /// let map: HashMap<&str, &str> = HashMap::new();
    /// assert!(map.is_empty());
    /// ```
    pub fn new() -> Self {
        Self(DashMap::new())
    }

    /// 插入一个键值对
    ///
    /// 如果键已存在，将替换旧值并返回旧值。
    ///
    /// # 参数
    ///
    /// * `key`: 键
    /// * `value`: 值
    ///
    /// # 返回值
    ///
    /// - 如果键不存在，返回 `None`
    /// - 如果键已存在，返回 `Some(旧值)`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::HashMap;
    ///
    /// let map = HashMap::new();
    /// assert_eq!(map.insert("key", "value1"), None);
    /// assert_eq!(map.insert("key", "value2"), Some("value1"));
    /// ```
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }

    /// 获取键对应的值
    ///
    /// # 参数
    ///
    /// * `key`: 要查找的键
    ///
    /// # 返回值
    ///
    /// - 如果键存在，返回 `Some(值的引用包装器)`
    /// - 如果键不存在，返回 `None`
    ///
    /// # 注意
    ///
    /// 返回的是一个引用包装器，实现了 `Deref<Target = V>`，可以直接使用。
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::HashMap;
    ///
    /// let map = HashMap::new();
    /// map.insert("key", "value");
    ///
    /// if let Some(value_ref) = map.get(&"key") {
    ///     assert_eq!(*value_ref, "value");
    /// }
    /// ```
    pub fn get(&self, key: &K) -> Option<impl std::ops::Deref<Target = V> + '_> {
        self.0.get(key)
    }

    /// 移除一个键值对
    ///
    /// # 参数
    ///
    /// * `key`: 要移除的键
    ///
    /// # 返回值
    ///
    /// - 如果键存在，返回 `Some((键, 值))`
    /// - 如果键不存在，返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::HashMap;
    ///
    /// let map = HashMap::new();
    /// map.insert("key", "value");
    /// assert_eq!(map.remove(&"key"), Some(("key", "value")));
    /// assert_eq!(map.remove(&"key"), None);
    /// ```
    pub fn remove(&self, key: &K) -> Option<(K, V)> {
        self.0.remove(key)
    }

    /// 检查映射中是否包含某个键
    ///
    /// # 参数
    ///
    /// * `key`: 要检查的键
    ///
    /// # 返回值
    ///
    /// - 如果键存在，返回 `true`
    /// - 如果键不存在，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::HashMap;
    ///
    /// let map = HashMap::new();
    /// map.insert("key", "value");
    /// assert!(map.contains_key(&"key"));
    /// assert!(!map.contains_key(&"nonexistent"));
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.0.contains_key(key)
    }

    /// 获取映射中键值对的数量
    ///
    /// # 返回值
    ///
    /// 返回映射中键值对的数量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::HashMap;
    ///
    /// let map = HashMap::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert("key1", "value1");
    /// map.insert("key2", "value2");
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 检查映射是否为空
    ///
    /// # 返回值
    ///
    /// - 如果映射为空，返回 `true`
    /// - 如果映射不为空，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::HashMap;
    ///
    /// let map: HashMap<&str, &str> = HashMap::new();
    /// assert!(map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 清空映射中的所有键值对
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::HashMap;
    ///
    /// let map = HashMap::new();
    /// map.insert("key", "value");
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&self) {
        self.0.clear();
    }
}

impl<K, V> Default for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

/// String 到 JSON Value 的 HashMap 类型别名
///
/// 这是一个常用的类型别名，用于存储字符串键到 JSON 值的映射。
/// 适合存储动态类型的配置、选项等。
///
/// # 示例
///
/// ```
/// use rf_container::StrAnyMap;
/// use serde_json::json;
///
/// let mut map = StrAnyMap::new();
/// map.insert("name".to_string(), json!("Alice"));
/// map.insert("age".to_string(), json!(30));
/// ```
pub type StrAnyMap = HashMap<String, Value>;

/// String 到 String 的 HashMap 类型别名
///
/// 用于存储字符串键到字符串值的映射。
/// 适合存储纯文本配置、选项等。
///
/// # 示例
///
/// ```
/// use rf_container::StrStrMap;
///
/// let mut map = StrStrMap::new();
/// map.insert("username".to_string(), "alice".to_string());
/// map.insert("email".to_string(), "alice@example.com".to_string());
/// ```
pub type StrStrMap = HashMap<String, String>;

/// String 到 i64 的 HashMap 类型别名
///
/// 用于存储字符串键到 64 位整数值的映射。
/// 适合存储计数器、统计信息等。
///
/// # 示例
///
/// ```
/// use rf_container::StrIntMap;
///
/// let mut map = StrIntMap::new();
/// map.insert("counter".to_string(), 100);
/// map.insert("total".to_string(), 1000);
/// ```
pub type StrIntMap = HashMap<String, i64>;

/// 有序映射包装器
///
/// 封装了 `IndexMap`，提供保持插入顺序的键值存储。
/// 与标准库的 `HashMap` 不同，`OrderedMap` 会按照插入顺序迭代键值对。
///
/// # 字段
///
/// - `0`: 内部的 `IndexMap<K, V>`，存储键值对
///
/// # 类型参数
///
/// * `K`: 键的类型，必须实现 `Hash` 和 `Eq` trait
/// * `V`: 值的类型
///
/// # 线程安全
///
/// `OrderedMap` **不是**线程安全的。如果需要线程安全，请使用 `HashMap`。
///
/// # 示例
///
/// ```
/// use rf_container::OrderedMap;
///
/// let mut map = OrderedMap::new();
/// map.insert("a", 1);
/// map.insert("b", 2);
/// map.insert("c", 3);
///
/// // 遍历顺序是插入顺序
/// let mut iter = map.iter();
/// assert_eq!(iter.next(), Some((&"a", &1)));
/// assert_eq!(iter.next(), Some((&"b", &2)));
/// assert_eq!(iter.next(), Some((&"c", &3)));
/// ```
#[derive(Debug, Clone)]
pub struct OrderedMap<K, V>(IndexMap<K, V>);

impl<K: Hash + Eq, V> OrderedMap<K, V> {
    /// 创建一个新的空有序映射
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `OrderedMap` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::OrderedMap;
    ///
    /// let map: OrderedMap<&str, i32> = OrderedMap::new();
    /// assert!(map.is_empty());
    /// ```
    pub fn new() -> Self {
        Self(IndexMap::new())
    }

    /// 插入一个键值对
    ///
    /// 如果键已存在，将替换旧值并返回旧值。
    /// 新键值对会插入到映射的末尾。
    ///
    /// # 参数
    ///
    /// * `key`: 键
    /// * `value`: 值
    ///
    /// # 返回值
    ///
    /// - 如果键不存在，返回 `None`
    /// - 如果键已存在，返回 `Some(旧值)`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::OrderedMap;
    ///
    /// let mut map = OrderedMap::new();
    /// assert_eq!(map.insert("key", 1), None);
    /// assert_eq!(map.insert("key", 2), Some(1));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }

    /// 获取键对应的值
    ///
    /// # 参数
    ///
    /// * `key`: 要查找的键
    ///
    /// # 返回值
    ///
    /// - 如果键存在，返回 `Some(&V)`
    /// - 如果键不存在，返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::OrderedMap;
    ///
    /// let mut map = OrderedMap::new();
    /// map.insert("key", 42);
    /// assert_eq!(map.get(&"key"), Some(&42));
    /// assert_eq!(map.get(&"nonexistent"), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }

    /// 移除一个键值对
    ///
    /// # 参数
    ///
    /// * `key`: 要移除的键
    ///
    /// # 返回值
    ///
    /// - 如果键存在，返回 `Some(值)`
    /// - 如果键不存在，返回 `None`
    ///
    /// # 注意
    ///
    /// 使用 `shift_remove` 方法，会保持其他键的相对顺序。
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::OrderedMap;
    ///
    /// let mut map = OrderedMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    /// assert_eq!(map.remove(&"b"), Some(2));
    /// assert_eq!(map.remove(&"b"), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0.shift_remove(key)
    }

    /// 检查映射中是否包含某个键
    ///
    /// # 参数
    ///
    /// * `key`: 要检查的键
    ///
    /// # 返回值
    ///
    /// - 如果键存在，返回 `true`
    /// - 如果键不存在，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::OrderedMap;
    ///
    /// let mut map = OrderedMap::new();
    /// map.insert("key", 42);
    /// assert!(map.contains_key(&"key"));
    /// assert!(!map.contains_key(&"nonexistent"));
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.0.contains_key(key)
    }

    /// 获取映射中键值对的数量
    ///
    /// # 返回值
    ///
    /// 返回映射中键值对的数量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::OrderedMap;
    ///
    /// let mut map = OrderedMap::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert("key1", 1);
    /// map.insert("key2", 2);
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 检查映射是否为空
    ///
    /// # 返回值
    ///
    /// - 如果映射为空，返回 `true`
    /// - 如果映射不为空，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::OrderedMap;
    ///
    /// let map: OrderedMap<&str, i32> = OrderedMap::new();
    /// assert!(map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 清空映射中的所有键值对
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::OrderedMap;
    ///
    /// let mut map = OrderedMap::new();
    /// map.insert("key", 42);
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<K: Hash + Eq, V> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
