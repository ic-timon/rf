//! # queue 模块
//!
//! queue 模块 - 队列容器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 队列容器
//!
//! 提供线程安全的队列 (`Queue`)，基于 `crossbeam` 的 `SegQueue` 实现。
//! 支持多线程并发地 push 和 pop 操作，适用于生产者-消费者场景。
//!
//! # 特性
//!
//! - 线程安全：多个线程可以同时 push 和 pop
//! - 无锁实现：使用 SegQueue（分段队列）实现高性能并发
//! - Clone 支持：可以克隆队列引用在多个线程间共享
//! - FIFO 顺序：先进先出
//!
//! # 示例
//!
//! ```
//! use rf_container::Queue;
//! use std::thread;
//!
//! let queue = Queue::new();
//!
//! // 生产者线程
//! // let queue_producer = queue.clone();
//! // thread::spawn(move || {
//! //     queue_producer.push(1);
//! //     queue_producer.push(2);
//! // });
//!
//! // 消费者线程
//! // thread::spawn(move || {
//! //     while let Some(value) = queue.pop() {
//! //         println!("Got: {}", value);
//! //     }
//! // });
//! ```

use crossbeam::queue::SegQueue;
use std::sync::Arc;

/// 线程安全的队列包装器
///
/// 封装了 `crossbeam::queue::SegQueue`，提供线程安全的 FIFO 队列操作。
/// 内部使用 `Arc` 实现，支持在线程之间克隆和共享。
///
/// # 字段
///
/// - `0`: 内部的 `Arc<SegQueue<T>>`，提供线程安全的队列操作
///
/// # 类型参数
///
/// * `T`: 队列中存储的元素类型
///
/// # 示例
///
/// ```
/// use rf_container::Queue;
///
/// let queue = Queue::new();
/// queue.push(42);
/// assert_eq!(queue.pop(), Some(42));
/// ```
#[derive(Debug, Clone)]
pub struct Queue<T>(Arc<SegQueue<T>>);

impl<T> Queue<T> {
    /// 创建一个新的空队列
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `Queue` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Queue;
    ///
    /// let queue: Queue<i32> = Queue::new();
    /// assert!(queue.is_empty());
    /// ```
    pub fn new() -> Self {
        Self(Arc::new(SegQueue::new()))
    }

    /// 向队列中推入一个元素
    ///
    /// 此操作是线程安全的，多个线程可以同时 push。
    ///
    /// # 参数
    ///
    /// * `value`: 要推入的元素值
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Queue;
    ///
    /// let queue = Queue::new();
    /// queue.push(1);
    /// queue.push(2);
    /// ```
    pub fn push(&self, value: T) {
        self.0.push(value);
    }

    /// 从队列中弹出一个元素
    ///
    /// 此操作是线程安全的，多个线程可以同时 pop。
    /// 如果队列为空，返回 `None`。
    ///
    /// # 返回值
    ///
    /// 如果队列不为空，返回 `Some(T)`，否则返回 `None`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Queue;
    ///
    /// let queue = Queue::new();
    /// queue.push(1);
    /// queue.push(2);
    /// assert_eq!(queue.pop(), Some(1));
    /// assert_eq!(queue.pop(), Some(2));
    /// assert_eq!(queue.pop(), None);
    /// ```
    pub fn pop(&self) -> Option<T> {
        self.0.pop()
    }

    /// 检查队列是否为空
    ///
    /// # 注意
    ///
    /// 在多线程环境下，此方法的返回值可能在使用时已经过时。
    /// 如果需要严格的原子性检查-操作，应该使用 `pop()` 并处理 `None` 情况。
    ///
    /// # 返回值
    ///
    /// 如果队列为空返回 `true`，否则返回 `false`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Queue;
    ///
    /// let queue: Queue<i32> = Queue::new();
    /// assert!(queue.is_empty());
    /// queue.push(1);
    /// assert!(!queue.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 获取队列的长度（近似值）
    ///
    /// # 注意
    ///
    /// 在多线程环境下，此方法的返回值只是一个近似值，
    /// 因为在其他线程中可能同时进行 push 或 pop 操作。
    ///
    /// # 返回值
    ///
    /// 返回队列中元素的大致数量
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Queue;
    ///
    /// let queue = Queue::new();
    /// queue.push(1);
    /// queue.push(2);
    /// assert_eq!(queue.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}
