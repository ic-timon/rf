//! # sel
//!
//! sel 模块 - 负载均衡选择器
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 负载均衡选择器
//!
//! 提供多种负载均衡策略，用于从候选服务器列表中选择一个服务器。
//!
//! # 主要功能
//!
//! - 支持多种负载均衡策略：轮询、随机、加权、最少连接
//! - 可扩展的 Selector trait，可以实现自定义策略
//!
//! # 使用示例
//!
//! ```ignore
//! use rf_net::sel::{RoundRobinSelector, Selector};
//!
//! // 创建轮询选择器
//! let mut selector = RoundRobinSelector::new();
//! let servers = vec!["server1", "server2", "server3"];
//!
//! // 轮询选择服务器
//! if let Some(server) = selector.select(&servers) {
//!     println!("选择的服务器: {}", server);
//! }
//! ```

/// 负载均衡策略
///
/// 定义了常见的负载均衡策略类型
#[derive(Debug, Clone, Copy)]
pub enum Strategy {
    /// 轮询（Round Robin）：按顺序依次选择每个服务器，实现请求的平均分配
    RoundRobin,
    /// 随机（Random）：随机选择一个服务器
    Random,
    /// 加权（Weighted）：根据服务器权重进行选择
    Weighted,
    /// 最少连接（Least Connection）：选择当前连接数最少的服务器
    LeastConnection,
}

/// 选择器 trait
///
/// 定义了从候选列表中选择一个项目的接口
///
/// # 类型参数
///
/// - `T`: 候选项目的类型
///
/// # 示例
///
/// ```ignore
/// struct MySelector;
///
/// impl Selector<String> for MySelector {
///     fn select<'a>(&mut self, items: &'a [String]) -> Option<&'a String> {
///         items.first()
///     }
/// }
/// ```
pub trait Selector<T> {
    /// 从候选列表中选择一个项目
    ///
    /// # 参数
    ///
    /// - `items`: 候选项目列表
    ///
    /// # 返回值
    ///
    /// - `Some(&T)`: 选中的项目
    /// - `None`: 列表为空或无法选择
    fn select<'a>(&mut self, items: &'a [T]) -> Option<&'a T>;
}

/// 轮询选择器
///
/// 实现轮询策略，依次选择每个候选项目。
/// 当到达列表末尾时，循环回到开头。
///
/// # 字段
///
/// - `index`: 当前选择的索引
///
/// # 示例
///
/// ```ignore
/// let mut selector = RoundRobinSelector::new();
/// let items = vec!["a", "b", "c"];
///
/// assert_eq!(selector.select(&items), Some(&"a"));
/// assert_eq!(selector.select(&items), Some(&"b"));
/// assert_eq!(selector.select(&items), Some(&"c"));
/// assert_eq!(selector.select(&items), Some(&"a")); // 循环回到开头
/// ```
pub struct RoundRobinSelector {
    index: usize,
}

impl RoundRobinSelector {
    /// 创建一个新的轮询选择器
    ///
    /// # 返回值
    ///
    /// 返回一个索引初始化为 0 的轮询选择器
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl<T> Selector<T> for RoundRobinSelector {
    /// 使用轮询策略选择一个项目
    ///
    /// # 算法
    ///
    /// 1. 如果列表为空，返回 None
    /// 2. 使用 `index % items.len()` 计算当前索引
    /// 3. 返回对应索引的项目
    /// 4. 递增索引到下一个位置
    fn select<'a>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            return None;
        }
        let item = items.get(self.index % items.len());
        self.index += 1;
        item
    }
}

impl Default for RoundRobinSelector {
    /// 创建默认的轮询选择器
    fn default() -> Self {
        Self::new()
    }
}
