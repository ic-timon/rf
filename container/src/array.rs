//! # array 模块
//!
//! array 模块 - 数组容器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 数组容器
//!
//! 提供基于 `SmallVec` 的小向量数组 (`Array`)。
//! SmallVec 是一个优化的小向量实现，当元素数量较少时（默认 8 个），
//! // 会将数据存储在栈上而不是堆上，从而提高性能并减少内存分配。
//!
//! # 特性
//!
//! - 栈上存储：对于少量元素（默认 8 个），数据存储在栈上
//! - 自动降级：当元素超过容量时，自动切换到堆分配
//! - 零开销抽象：与标准 Vec 相似的 API，但在小数据集时更快
//! - 内存友好：避免小数据集的堆分配开销
//!
//! # 示例
//!
//! ```
//! use rf_container::Array;
//!
//! // 创建数组
//! let mut arr = Array::new();
//! arr.push(1);
//! arr.push(2);
//! arr.push(3);
//!
//! // 获取元素
//! assert_eq!(arr.get(0), Some(&1));
//!
//! // 弹出元素
//! // assert_eq!(arr.pop(), Some(3));
//! ```
//!
//! # 性能说明
//!
//! SmallVec 适合以下场景：
//! - 大部分情况下只存储少量元素
//! - 需要避免频繁的堆分配
//! - 元素数量偶尔会超过内联容量
//!
//! 如果大多数情况下元素数量都超过 8 个，建议直接使用 `Vec<T>`。

use smallvec::SmallVec;

/// 小向量数组容器
///
/// 封装了 `SmallVec`，内联容量为 8 个元素。
/// 对于 8 个或更少的元素，数据存储在栈上；超过 8 个时自动切换到堆分配。
///
/// # 字段
///
/// - `0`: 内部的 `SmallVec<[T; 8]>`，实际存储数据
///
/// # 类型参数
///
/// * `T`: 数组中存储的元素类型
///
/// # 示例
///
/// ```
/// use rf_container::Array;
///
/// let mut arr = Array::new();
/// arr.push(1);
/// arr.push(2);
/// assert_eq!(arr.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct Array<T>(SmallVec<[T; 8]>);

impl<T> Array<T> {
    /// 创建一个新的空数组
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `Array` 实例，初始容量为 8 个元素（栈上）
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Array;
    ///
    /// let arr: Array<i32> = Array::new();
    /// assert!(arr.is_empty());
    /// ```
    pub fn new() -> Self {
        Self(SmallVec::new())
    }

    /// 创建一个具有指定容量的数组
    ///
    /// # 参数
    ///
    /// * `capacity`: 初始容量
    ///
    /// # 注意
    ///
    /// 如果 `capacity` <= 8，数据会存储在栈上；否则会进行堆分配。
    ///
    /// # 返回值
    ///
    /// 返回一个具有指定容量的 `Array` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Array;
    ///
    /// let arr: Array<i32> = Array::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self(SmallVec::with_capacity(capacity))
    }

    /// 向数组末尾推入一个元素
    ///
    /// # 参数
    ///
    /// * `value`: 要推入的元素值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Array;
    ///
    /// let mut arr = Array::new();
    /// arr.push(1);
    /// arr.push(2);
    /// ```
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    /// 从数组末尾弹出一个元素
    ///
    /// # 返回值
    ///
    /// 如果数组不为空，返回 `Some(T)`，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Array;
    ///
    /// let mut arr = Array::new();
    /// arr.push(1);
    /// assert_eq!(arr.pop(), Some(1));
    /// assert_eq!(arr.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    /// 获取指定索引位置的元素
    ///
    /// # 参数
    ///
    /// * `index`: 元素的索引位置（从 0 开始）
    ///
    /// # 返回值
    ///
    /// 如果索引有效，返回 `Some(&T)`，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Array;
    ///
    /// let mut arr = Array::new();
    /// arr.push(1);
    /// arr.push(2);
    /// assert_eq!(arr.get(0), Some(&1));
    /// assert_eq!(arr.get(1), Some(&2));
    /// assert_eq!(arr.get(10), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    /// 获取数组的长度
    ///
    /// # 返回值
    ///
    /// 返回数组中元素的数量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Array;
    ///
    /// let mut arr = Array::new();
    /// assert_eq!(arr.len(), 0);
    /// arr.push(1);
    /// assert_eq!(arr.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 检查数组是否为空
    ///
    /// # 返回值
    ///
    /// 如果数组为空返回 `true`，否则返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Array;
    ///
    /// let arr: Array<i32> = Array::new();
    /// assert!(arr.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 清空数组中的所有元素
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Array;
    ///
    /// let mut arr = Array::new();
    /// arr.push(1);
    /// arr.push(2);
    /// arr.clear();
    /// assert!(arr.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}
