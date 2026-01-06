//! # lib
//!
//! lib 模块 - RF 框架的核心库
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! RF Core Module
//!
//! 提供 RF 框架的核心类型、trait 和通用功能。
//!
//! # 模块结构
//!
//! - `types`: 定义了各种常用的类型别名，包括 Map、List 和 Var 等容器类型
//! - `traits`: 定义了框架的核心 trait，包括 ToString、Clone、Compare 和 Hash
//!
//! # 使用示例
//!
//! ```rust
//! use rframe_core::{Map, List, Var};
//!
//! // 创建一个 Map
//! let mut map: Map = Map::new();
//! map.insert("key".to_string(), serde_json::json!("value"));
//!
//! // 创建一个 List
//! let list: List = vec![map];
//! ```
//!
//! # 主要功能
//!
//! - **类型别名**: 为常用的 HashMap 和 Vec 类型提供简短的别名
//! - **核心 Trait**: 定义对象转换、克隆、比较和哈希的基本接口
//! - **通用接口**: 提供统一的变量类型 Var (Value) 用于处理动态数据

pub mod types;
pub mod traits;

pub use types::*;
pub use traits::*;

