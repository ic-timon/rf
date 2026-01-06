//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # RF 容器模块
//!
//! 提供各种数据结构容器，包括：
//! - 原子类型包装器（type）：线程安全的基本类型包装
//! - 通用变量类型（var）：类似 GoFrame 的 gvar.Var
//! - 映射容器（map）：HashMap 和 OrderedMap
//! - 数组容器（array）：基于 SmallVec 的小向量数组
//! - 链表容器（list）：双向链表
//! - 集合容器（set）：有序集合
//! - 队列容器（queue）：线程安全队列
//! - 环形缓冲区（ring）：固定大小的环形缓冲区
//! - 对象池（pool）：对象池管理器
//! - 树容器（tree）：通用树结构
//!
//! # 示例
//!
//! ```
//! use rf_container::{Queue, HashMap, Var};
//!
//! // 创建线程安全队列
//! let queue = Queue::new();
//! queue.push(42);
//!
//! // 创建线程安全的 HashMap
//! let map = HashMap::new();
//! map.insert("key".to_string(), "value".to_string());
//!
//! // 创建通用变量
//! let var = Var::new("Hello");
//! println!("{}", var.string());
//! ```

pub mod r#type;
pub mod r#var;
pub mod map;
pub mod array;
pub mod list;
pub mod set;
pub mod queue;
pub mod ring;
pub mod pool;
pub mod tree;

pub use r#type::*;
pub use r#var::*;
pub use map::*;
pub use array::*;
pub use list::*;
pub use set::*;
pub use queue::*;
pub use ring::*;
pub use pool::*;
pub use tree::*;

