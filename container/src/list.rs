//! # list 模块
//!
//! list 模块 - 链表容器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 链表容器
//!
//! 提供双向链表 (`List`) 的封装，基于标准库的 `LinkedList` 实现。
//! 支持在链表头部和尾部高效地插入和删除元素。
//!
//! # 特性
//!
//! - O(1) 时间复杂度的头部和尾部插入/删除
//! - 双向迭代支持
//! - 适合频繁在两端操作的场景
//!
//! # 示例
//!
//! ```
//! use rf_container::List;
//!
//! let mut list = List::new();
//!
//! // 在尾部插入元素
//! list.push_back(1);
//! list.push_back(2);
//! list.push_back(3);
//!
//! // 在头部插入元素
//! list.push_front(0);
//!
//! // 从头部弹出元素
//! assert_eq!(list.pop_front(), Some(0));
//!
//! // 从尾部弹出元素
//! assert_eq!(list.pop_back(), Some(3));
//! ```

use std::collections::LinkedList;

/// 双向链表包装器
///
/// 封装了标准库的 `LinkedList`，提供简洁的 API 用于操作双向链表。
///
/// # 字段
///
/// - `0`: 内部的 `LinkedList<T>`，存储链表数据
///
/// # 类型参数
///
/// * `T`: 链表中存储的元素类型
///
/// # 示例
///
/// ```
/// use rf_container::List;
///
/// let mut list = List::new();
/// list.push_back(1);
/// list.push_back(2);
/// assert_eq!(list.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct List<T>(LinkedList<T>);

impl<T> List<T> {
    /// 创建一个新的空链表
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `List` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::List;
    ///
    /// let list: List<i32> = List::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn new() -> Self {
        Self(LinkedList::new())
    }

    /// 在链表头部插入一个元素
    ///
    /// # 参数
    ///
    /// * `value`: 要插入的元素值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_front(0);
    /// ```
    pub fn push_front(&mut self, value: T) {
        self.0.push_front(value);
    }

    /// 在链表尾部插入一个元素
    ///
    /// # 参数
    ///
    /// * `value`: 要插入的元素值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// ```
    pub fn push_back(&mut self, value: T) {
        self.0.push_back(value);
    }

    /// 从链表头部弹出一个元素
    ///
    /// # 返回值
    ///
    /// 如果链表不为空，返回 `Some(T)`，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    /// 从链表尾部弹出一个元素
    ///
    /// # 返回值
    ///
    /// 如果链表不为空，返回 `Some(T)`，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }

    /// 获取链表的长度
    ///
    /// # 返回值
    ///
    /// 返回链表中元素的数量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::List;
    ///
    /// let mut list = List::new();
    /// assert_eq!(list.len(), 0);
    /// list.push_back(1);
    /// assert_eq!(list.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 检查链表是否为空
    ///
    /// # 返回值
    ///
    /// 如果链表为空返回 `true`，否则返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::List;
    ///
    /// let list: List<i32> = List::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 清空链表中的所有元素
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::List;
    ///
    /// let mut list = List::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.clear();
    /// assert!(list.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}
