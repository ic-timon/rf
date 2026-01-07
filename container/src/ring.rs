//! # ring 模块
//!
//! ring 模块 - 环形缓冲区容器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 环形缓冲区容器
//!
//! 提供固定大小的环形缓冲区 (`Ring`) 和线程安全的环形缓冲区 (`ThreadSafeRing`)。
//!
//! # 特性
//!
//! ## Ring
//! - 固定容量：创建时指定容量，不会增长
//! - FIFO 行为：满时自动覆盖最旧的元素
//! - 高效操作：使用 VecDeque 实现，O(1) 的推入和弹出
//!
//! ## ThreadSafeRing
//! - 线程安全：使用 Mutex 包装，可在多线程环境中使用
//! - 共享访问：通过 Arc 共享，支持 Clone
//!
//! # 使用场景
//!
//! - 日志缓冲：记录最近的 N 条日志
//! - 数据流处理：滑动窗口数据处理
//! - 事件队列：固定大小的事件历史
//! - 性能监控：保留最近的性能指标
//!
//! # 示例
//!
//! ```
//! use rf_container::Ring;
//!
//! // 创建容量为 3 的环形缓冲区
//! let mut ring = Ring::new(3);
//!
//! ring.push(1);
//! ring.push(2);
//! ring.push(3);
//! ring.push(4); // 覆盖 1
//!
//! assert_eq!(ring.pop(), Some(2)); // 弹出最旧的
//! ```

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// 环形缓冲区
///
/// 使用 `VecDeque` 实现的固定大小缓冲区。
/// 当缓冲区满时，新元素会自动覆盖最旧的元素（FIFO 行为）。
///
/// # 字段
///
/// - `buffer`: 内部的 `VecDeque<T>`，存储实际数据
/// - `capacity`: 缓冲区的最大容量
///
/// # 类型参数
///
/// * `T`: 缓冲区中存储的元素类型
///
/// # 示例
///
/// ```
/// use rf_container::Ring;
///
/// let mut ring = Ring::new(3);
/// ring.push(1);
/// ring.push(2);
/// assert_eq!(ring.len(), 2);
/// ```
pub struct Ring<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> Ring<T> {
    /// 创建一个具有指定容量的环形缓冲区
    ///
    /// # 参数
    ///
    /// * `capacity`: 缓冲区的最大容量
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `Ring` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let ring: Ring<i32> = Ring::new(5);
    /// assert_eq!(ring.capacity(), 5);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// 向环形缓冲区推入一个值
    ///
    /// 如果缓冲区已满，最旧的元素会被移除（环形缓冲区行为）。
    ///
    /// # 参数
    ///
    /// * `value`: 要推入的值
    ///
    /// # 返回值
    ///
    /// 总是返回 `Ok(())`，为了保持 API 一致性
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(2);
    /// ring.push(1);
    /// ring.push(2);
    /// ring.push(3); // 1 被覆盖
    /// assert_eq!(ring.peek(), Some(&2));
    /// ```
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.buffer.len() >= self.capacity {
            // 如果已满，移除最旧的元素
            self.buffer.pop_front();
        }
        self.buffer.push_back(value);
        Ok(())
    }

    /// 尝试推入一个值，如果缓冲区已满则返回错误
    ///
    /// 当你不想覆盖现有元素时，此方法很有用。
    ///
    /// # 参数
    ///
    /// * `value`: 要推入的值
    ///
    /// # 返回值
    ///
    /// - 成功推入返回 `Ok(())`
    /// - 缓冲区已满返回 `Err(value)`，返回未插入的值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(2);
    /// ring.push(1);
    /// ring.push(2);
    /// assert!(ring.try_push(3).is_err()); // 已满
    /// ```
    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.buffer.len() >= self.capacity {
            return Err(value);
        }
        self.buffer.push_back(value);
        Ok(())
    }

    /// 从环形缓冲区弹出一个值（移除最旧的元素）
    ///
    /// # 返回值
    ///
    /// - 如果缓冲区不为空，返回 `Some(最旧的元素)`
    /// - 如果缓冲区为空，返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(3);
    /// ring.push(1);
    /// ring.push(2);
    /// assert_eq!(ring.pop(), Some(1));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.buffer.pop_front()
    }

    /// 查看最旧的元素但不移除
    ///
    /// # 返回值
    ///
    /// - 如果缓冲区不为空，返回 `Some(&最旧的元素)`
    /// - 如果缓冲区为空，返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(3);
    /// ring.push(1);
    /// ring.push(2);
    /// assert_eq!(ring.peek(), Some(&1));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.buffer.front()
    }

    /// 查看最新的元素但不移除
    ///
    /// # 返回值
    ///
    /// - 如果缓冲区不为空，返回 `Some(&最新的元素)`
    /// - 如果缓冲区为空，返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(3);
    /// ring.push(1);
    /// ring.push(2);
    /// assert_eq!(ring.peek_back(), Some(&2));
    /// ```
    pub fn peek_back(&self) -> Option<&T> {
        self.buffer.back()
    }

    /// 检查环形缓冲区是否为空
    ///
    /// # 返回值
    ///
    /// - 如果缓冲区为空，返回 `true`
    /// - 如果缓冲区不为空，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let ring: Ring<i32> = Ring::new(3);
    /// assert!(ring.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// 检查环形缓冲区是否已满
    ///
    /// # 返回值
    ///
    /// - 如果缓冲区已满，返回 `true`
    /// - 如果缓冲区未满，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(2);
    /// ring.push(1);
    /// ring.push(2);
    /// assert!(ring.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }

    /// 获取环形缓冲区的长度
    ///
    /// # 返回值
    ///
    /// 返回当前缓冲区中元素的数量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(3);
    /// ring.push(1);
    /// ring.push(2);
    /// assert_eq!(ring.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// 获取环形缓冲区的容量
    ///
    /// # 返回值
    ///
    /// 返回缓冲区的最大容量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let ring = Ring::<i32>::new(5);
    /// assert_eq!(ring.capacity(), 5);
    /// ```
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 清空环形缓冲区中的所有元素
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(3);
    /// ring.push(1);
    /// ring.push(2);
    /// ring.clear();
    /// assert!(ring.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// 获取环形缓冲区的迭代器（从最旧到最新）
    ///
    /// # 返回值
    ///
    /// 返回一个按元素插入顺序（从最旧到最新）迭代的迭代器
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(3);
    /// ring.push(1);
    /// ring.push(2);
    /// ring.push(3);
    ///
    /// let values: Vec<&i32> = ring.iter().collect();
    /// assert_eq!(values, vec![&1, &2, &3]);
    /// ```
    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.buffer.iter()
    }

    /// 获取环形缓冲区的可变迭代器
    ///
    /// # 返回值
    ///
    /// 返回一个可变迭代器，允许修改元素
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(3);
    /// ring.push(1);
    /// ring.push(2);
    ///
    /// for val in ring.iter_mut() {
    ///     *val *= 2;
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> std::collections::vec_deque::IterMut<'_, T> {
        self.buffer.iter_mut()
    }

    /// 一次推入多个值
    ///
    /// # 参数
    ///
    /// * `values`: 要推入的值迭代器
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(5);
    /// ring.push_batch(vec![1, 2, 3, 4]);
    /// ```
    pub fn push_batch(&mut self, values: impl IntoIterator<Item = T>) {
        for value in values {
            let _ = self.push(value);
        }
    }

    /// 一次弹出多个值
    ///
    /// # 参数
    ///
    /// * `count`: 要弹出的元素数量
    ///
    /// # 返回值
    ///
    /// 返回一个包含被弹出元素的向量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(5);
    /// ring.push_batch(vec![1, 2, 3, 4, 5]);
    /// let popped = ring.pop_batch(3);
    /// assert_eq!(popped, vec![1, 2, 3]);
    /// ```
    pub fn pop_batch(&mut self, count: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(count.min(self.buffer.len()));
        for _ in 0..count {
            if let Some(value) = self.pop() {
                result.push(value);
            } else {
                break;
            }
        }
        result
    }

    /// 获取环形缓冲区的一个切片（作为向量返回）
    ///
    /// # 参数
    ///
    /// * `start`: 起始索引（从 0 开始）
    /// * `end`: 结束索引（不包含）
    ///
    /// # 返回值
    ///
    /// 返回一个包含指定范围内元素引用的向量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(5);
    /// ring.push_batch(vec![1, 2, 3, 4, 5]);
    /// let slice = ring.slice(1, 3);
    /// assert_eq!(slice, vec![&2, &3]);
    /// ```
    pub fn slice(&self, start: usize, end: usize) -> Vec<&T> {
        let len = self.buffer.len();
        let start = start.min(len);
        let end = end.min(len);

        self.buffer
            .iter()
            .skip(start)
            .take(end - start)
            .collect()
    }

    /// 获取环形缓冲区的一个范围内的值
    ///
    /// # 参数
    ///
    /// * `start`: 起始索引（从 0 开始）
    /// * `end`: 结束索引（不包含）
    ///
    /// # 返回值
    ///
    /// 返回一个包含指定范围内元素引用的向量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(5);
    /// ring.push_batch(vec![1, 2, 3, 4, 5]);
    /// let range = ring.range(1, 4);
    /// assert_eq!(range, vec![&2, &3, &4]);
    /// ```
    pub fn range(&self, start: usize, end: usize) -> Vec<&T> {
        let len = self.buffer.len();
        let start = start.min(len);
        let end = end.min(len);

        self.buffer
            .iter()
            .skip(start)
            .take(end - start)
            .collect()
    }

    /// 调整环形缓冲区的容量
    ///
    /// 如果新容量小于当前元素数量，最旧的元素会被移除。
    ///
    /// # 参数
    ///
    /// * `new_capacity`: 新的容量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(5);
    /// ring.push_batch(vec![1, 2, 3, 4, 5]);
    /// ring.resize(3);
    /// assert_eq!(ring.len(), 3);
    /// assert_eq!(ring.peek(), Some(&3));
    /// ```
    pub fn resize(&mut self, new_capacity: usize) {
        if new_capacity < self.buffer.len() {
            // 移除最旧的元素
            let remove_count = self.buffer.len() - new_capacity;
            for _ in 0..remove_count {
                self.buffer.pop_front();
            }
        }
        self.capacity = new_capacity;
        // VecDeque 没有直接的 resize 方法，所以创建一个新的
        let mut new_buffer = std::collections::VecDeque::with_capacity(new_capacity);
        for item in self.buffer.drain(..) {
            new_buffer.push_back(item);
        }
        self.buffer = new_buffer;
    }

    /// 获取环形缓冲区的统计信息
    ///
    /// # 返回值
    ///
    /// 返回一个 `RingStats` 结构体，包含：
    /// - `count`: 当前元素数量
    /// - `capacity`: 总容量
    /// - `utilization`: 使用率（0.0 到 1.0）
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Ring;
    ///
    /// let mut ring = Ring::new(10);
    /// ring.push_batch(vec![1, 2, 3]);
    /// let stats = ring.stats();
    /// assert_eq!(stats.count, 3);
    /// assert_eq!(stats.capacity, 10);
    /// assert_eq!(stats.utilization, 0.3);
    /// ```
    pub fn stats(&self) -> RingStats
    where
        T: Clone + std::cmp::Ord + std::ops::Add<Output = T> + std::ops::Div<Output = T> + From<usize>,
    {
        if self.buffer.is_empty() {
            return RingStats {
                count: 0,
                capacity: self.capacity,
                utilization: 0.0,
            };
        }

        let count = self.buffer.len();
        let utilization = count as f64 / self.capacity as f64;

        RingStats {
            count,
            capacity: self.capacity,
            utilization,
        }
    }
}

/// 环形缓冲区的统计信息
///
/// # 字段
///
/// - `count`: 当前元素数量
/// - `capacity`: 总容量
/// - `utilization`: 使用率（0.0 到 1.0）
#[derive(Debug, Clone)]
pub struct RingStats {
    pub count: usize,
    pub capacity: usize,
    pub utilization: f64,
}

impl<T> IntoIterator for Ring<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Ring<T> {
    type Item = &'a T;
    type IntoIter = std::collections::vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Ring<T> {
    type Item = &'a mut T;
    type IntoIter = std::collections::vec_deque::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter_mut()
    }
}

/// 线程安全的环形缓冲区
///
/// 使用 `Arc<Mutex<Ring<T>>>` 实现，可在多线程环境中安全使用。
/// 所有方法都是线程安全的，内部通过 Mutex 进行同步。
///
/// # 字段
///
/// - `inner`: 内部的 `Arc<Mutex<Ring<T>>>`，提供线程安全访问
///
/// # 类型参数
///
/// * `T`: 缓冲区中存储的元素类型
///
/// # 线程安全
///
/// `ThreadSafeRing` 是完全线程安全的，多个线程可以同时调用任意方法。
///
/// # 示例
///
/// ```
/// use rf_container::ThreadSafeRing;
/// use std::thread;
///
/// let ring = ThreadSafeRing::new(10);
/// let ring_clone = ring.clone();
///
/// thread::spawn(move || {
///     ring_clone.push(42);
/// });
/// ```
pub struct ThreadSafeRing<T> {
    inner: Arc<Mutex<Ring<T>>>,
}

impl<T> ThreadSafeRing<T> {
    /// 创建一个新的线程安全环形缓冲区
    ///
    /// # 参数
    ///
    /// * `capacity`: 缓冲区的最大容量
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `ThreadSafeRing` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring: ThreadSafeRing<i32> = ThreadSafeRing::new(5);
    /// assert_eq!(ring.capacity(), 5);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Ring::new(capacity))),
        }
    }

    /// 线程安全地推入一个值
    ///
    /// 如果缓冲区已满，最旧的元素会被移除。
    ///
    /// # 参数
    ///
    /// * `value`: 要推入的值
    ///
    /// # 返回值
    ///
    /// 总是返回 `Ok(())`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(3);
    /// ring.push(1);
    /// ring.push(2);
    /// ```
    pub fn push(&self, value: T) -> Result<(), T> {
        let mut buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing::push - this should not happen in normal operation");
        buffer.push(value)
    }

    /// 线程安全地尝试推入一个值
    ///
    /// # 参数
    ///
    /// * `value`: 要推入的值
    ///
    /// # 返回值
    ///
    /// - 成功推入返回 `Ok(())`
    /// - 缓冲区已满返回 `Err(value)`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(2);
    /// ring.push(1).ok();
    /// ring.push(2).ok();
    /// assert!(ring.try_push(3).is_err());
    /// ```
    pub fn try_push(&self, value: T) -> Result<(), T> {
        let mut buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.try_push(value)
    }

    /// 线程安全地弹出一个值
    ///
    /// # 返回值
    ///
    /// - 如果缓冲区不为空，返回 `Some(最旧的元素)`
    /// - 如果缓冲区为空，返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(3);
    /// ring.push(1).ok();
    /// assert_eq!(ring.pop(), Some(1));
    /// ```
    pub fn pop(&self) -> Option<T> {
        let mut buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.pop()
    }

    /// 线程安全地查看最旧的元素（需要 T: Clone）
    ///
    /// # 返回值
    ///
    /// - 如果缓冲区不为空，返回 `Some(最旧的元素的克隆)`
    /// - 如果缓冲区为空，返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(3);
    /// ring.push(42).ok();
    /// assert_eq!(ring.peek(), Some(42));
    /// ```
    pub fn peek(&self) -> Option<T>
    where
        T: Clone,
    {
        let buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.peek().cloned()
    }

    /// 线程安全地检查缓冲区是否为空
    ///
    /// # 返回值
    ///
    /// - 如果缓冲区为空，返回 `true`
    /// - 如果缓冲区不为空，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring: ThreadSafeRing<i32> = ThreadSafeRing::new(3);
    /// assert!(ring.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        let buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.is_empty()
    }

    /// 线程安全地检查缓冲区是否已满
    ///
    /// # 返回值
    ///
    /// - 如果缓冲区已满，返回 `true`
    /// - 如果缓冲区未满，返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(2);
    /// ring.push(1).ok();
    /// ring.push(2).ok();
    /// assert!(ring.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        let buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.is_full()
    }

    /// 线程安全地获取缓冲区长度
    ///
    /// # 返回值
    ///
    /// 返回当前元素数量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(3);
    /// ring.push(1).ok();
    /// ring.push(2).ok();
    /// assert_eq!(ring.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        let buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.len()
    }

    /// 线程安全地获取缓冲区容量
    ///
    /// # 返回值
    ///
    /// 返回最大容量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring: ThreadSafeRing<i32> = ThreadSafeRing::new(5);
    /// assert_eq!(ring.capacity(), 5);
    /// ```
    pub fn capacity(&self) -> usize {
        let buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.capacity()
    }

    /// 线程安全地清空缓冲区
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(3);
    /// ring.push(1).ok();
    /// ring.clear();
    /// assert!(ring.is_empty());
    /// ```
    pub fn clear(&self) {
        let mut buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.clear();
    }

    /// 线程安全地批量推入值
    ///
    /// # 参数
    ///
    /// * `values`: 要推入的值迭代器
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(5);
    /// ring.push_batch(vec![1, 2, 3]);
    /// ```
    pub fn push_batch(&self, values: impl IntoIterator<Item = T>) {
        let mut buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.push_batch(values);
    }

    /// 线程安全地批量弹出值
    ///
    /// # 参数
    ///
    /// * `count`: 要弹出的元素数量
    ///
    /// # 返回值
    ///
    /// 返回包含被弹出元素的向量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(5);
    /// ring.push_batch(vec![1, 2, 3, 4, 5]);
    /// let popped = ring.pop_batch(3);
    /// assert_eq!(popped, vec![1, 2, 3]);
    /// ```
    pub fn pop_batch(&self, count: usize) -> Vec<T> {
        let mut buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.pop_batch(count)
    }

    /// 线程安全地获取切片（需要 T: Clone）
    ///
    /// # 参数
    ///
    /// * `start`: 起始索引
    /// * `end`: 结束索引
    ///
    /// # 返回值
    ///
    /// 返回包含指定范围内元素克隆的向量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(5);
    /// ring.push_batch(vec![1, 2, 3, 4, 5]);
    /// let slice = ring.slice(1, 3);
    /// assert_eq!(slice, vec![2, 3]);
    /// ```
    pub fn slice(&self, start: usize, end: usize) -> Vec<T>
    where
        T: Clone,
    {
        let buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.range(start, end).into_iter().cloned().collect()
    }

    /// 线程安全地获取范围内的值（需要 T: Clone）
    ///
    /// # 参数
    ///
    /// * `start`: 起始索引
    /// * `end`: 结束索引
    ///
    /// # 返回值
    ///
    /// 返回包含指定范围内元素克隆的向量
    pub fn range(&self, start: usize, end: usize) -> Vec<T>
    where
        T: Clone,
    {
        self.slice(start, end)
    }

    /// 线程安全地调整容量
    ///
    /// # 参数
    ///
    /// * `new_capacity`: 新的容量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(5);
    /// ring.push_batch(vec![1, 2, 3, 4, 5]);
    /// ring.resize(3);
    /// assert_eq!(ring.len(), 3);
    /// ```
    pub fn resize(&self, new_capacity: usize) {
        let mut buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.resize(new_capacity);
    }

    /// 线程安全地获取统计信息
    ///
    /// # 返回值
    ///
    /// 返回 `RingStats` 结构体
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::ThreadSafeRing;
    ///
    /// let ring = ThreadSafeRing::new(10);
    /// ring.push_batch(vec![1, 2, 3]);
    /// let stats = ring.stats();
    /// assert_eq!(stats.count, 3);
    /// ```
    pub fn stats(&self) -> RingStats
    where
        T: Clone + std::cmp::Ord + std::ops::Add<Output = T> + std::ops::Div<Output = T> + From<usize>,
    {
        let buffer = self.inner.lock()
            .expect("Mutex poisoned in ThreadSafeRing - this should not happen in normal operation");
        buffer.stats()
    }
}

impl<T> Clone for ThreadSafeRing<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}
