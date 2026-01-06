//! # set 模块
//!
//! set 模块 - 集合容器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 集合容器
//!
//! 提供有序集合 (`Set`) 的封装，基于 `indexmap::IndexSet` 实现。
//! 有序集合会保持元素的插入顺序，同时确保元素的唯一性。
//!
//! # 特性
//!
//! - 插入顺序：保持元素的插入顺序
//! - 唯一性：自动去重，相同的元素只会保留一个
//! - 快速查找：O(1) 平均时间复杂度的查找操作
//! - 有序迭代：按照插入顺序迭代元素
//!
//! # 示例
//!
//! ```
//! use rf_container::Set;
//!
//! let mut set = Set::new();
//!
//! // 插入元素
//! set.insert(1);
//! set.insert(2);
//! set.insert(1); // 重复元素不会被插入
//!
//! assert_eq!(set.len(), 2);
//! assert!(set.contains(&1));
//! ```
//!
//! # 使用场景
//!
//! - 需要保持元素唯一性
//! - 需要保持插入顺序
//! - 需要快速查找元素是否存在

use indexmap::IndexSet;
use std::hash::Hash;

/// 有序集合包装器
///
/// 封装了 `indexmap::IndexSet`，提供有序且唯一的集合操作。
/// 与标准库的 `HashSet` 不同，`IndexSet` 会保持元素的插入顺序。
///
/// # 字段
///
/// - `0`: 内部的 `IndexSet<T>`，存储集合数据
///
/// # 类型参数
///
/// * `T`: 集合中存储的元素类型，必须实现 `Hash` 和 `Eq` trait
///
/// # 示例
///
/// ```
/// use rf_container::Set;
///
/// let mut set = Set::new();
/// set.insert("apple");
/// set.insert("banana");
/// assert_eq!(set.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct Set<T: Hash + Eq>(IndexSet<T>);

impl<T: Hash + Eq> Set<T> {
    /// 创建一个新的空集合
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `Set` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Set;
    ///
    /// let set: Set<i32> = Set::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn new() -> Self {
        Self(IndexSet::new())
    }

    /// 向集合中插入一个元素
    ///
    /// 如果集合中已存在相同的元素，则不会重复插入。
    ///
    /// # 参数
    ///
    /// * `value`: 要插入的元素值
    ///
    /// # 返回值
    ///
    /// - 如果元素不存在且成功插入，返回 `true`
    /// - 如果元素已存在，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Set;
    ///
    /// let mut set = Set::new();
    /// assert_eq!(set.insert(1), true);
    /// assert_eq!(set.insert(1), false); // 重复插入
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
        self.0.insert(value)
    }

    /// 从集合中移除一个元素
    ///
    /// # 参数
    ///
    /// * `value`: 要移除的元素的引用
    ///
    /// # 返回值
    ///
    /// - 如果元素存在且成功移除，返回 `true`
    /// - 如果元素不存在，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Set;
    ///
    /// let mut set = Set::new();
    /// set.insert(1);
    /// assert_eq!(set.remove(&1), true);
    /// assert_eq!(set.remove(&1), false); // 已经不存在
    /// ```
    pub fn remove(&mut self, value: &T) -> bool {
        self.0.swap_remove(value)
    }

    /// 检查集合中是否包含某个元素
    ///
    /// # 参数
    ///
    /// * `value`: 要查找的元素的引用
    ///
    /// # 返回值
    ///
    /// - 如果元素存在，返回 `true`
    /// - 如果元素不存在，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Set;
    ///
    /// let mut set = Set::new();
    /// set.insert(1);
    /// assert!(set.contains(&1));
    /// assert!(!set.contains(&2));
    /// ```
    pub fn contains(&self, value: &T) -> bool {
        self.0.contains(value)
    }

    /// 获取集合的长度
    ///
    /// # 返回值
    ///
    /// 返回集合中元素的数量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Set;
    ///
    /// let mut set = Set::new();
    /// assert_eq!(set.len(), 0);
    /// set.insert(1);
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 检查集合是否为空
    ///
    /// # 返回值
    ///
    /// 如果集合为空返回 `true`，否则返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Set;
    ///
    /// let set: Set<i32> = Set::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 清空集合中的所有元素
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Set;
    ///
    /// let mut set = Set::new();
    /// set.insert(1);
    /// set.insert(2);
    /// set.clear();
    /// assert!(set.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<T: Hash + Eq> Default for Set<T> {
    fn default() -> Self {
        Self::new()
    }
}
