//! # mod
//!
//! mod 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # Database ORM Module
//!
//! 数据库 ORM 核心模块，提供完整的数据访问层功能。
//!
//! ## 子模块
//!
//! - `model`: ORM 模型，提供表操作接口
//! - `query`: 查询构建器，支持类型安全的查询构建
//! - `transaction`: 事务管理，支持 ACID 事务
//! - `database`: 数据库连接和连接池管理
//! - `cache`: 查询结果缓存，提升查询性能
//! - `logger`: 查询日志记录和性能监控
//! - `pool_monitor`: 连接池状态监控和健康检查
//! - `replication`: 主从复制和读写分离
//! - `query_plan_cache`: 查询计划缓存优化

pub mod model;
pub mod query;
pub mod transaction;
pub mod database;
pub mod cache;
pub mod logger;
pub mod pool_monitor;
pub mod replication;
pub mod query_plan_cache;

pub use model::*;
pub use query::*;
pub use transaction::*;
pub use database::*;
pub use cache::*;
pub use logger::*;
pub use pool_monitor::*;
pub use replication::*;
pub use query_plan_cache::*;

