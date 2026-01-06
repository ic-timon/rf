//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # RF Database Module
//!
//! 提供 ORM 和数据库操作功能的模块。
//!
//! ## 主要功能
//!
//! - **数据库 ORM**: 提供面向对象的数据建模和查询接口
//! - **Redis 客户端**: 封装 Redis 操作，支持多种数据结构
//! - **查询构建器**: 类型安全的 SQL 查询构建
//! - **事务管理**: 支持数据库事务和缓存管理
//! - **连接池监控**: 监控数据库连接池状态和健康度
//! - - **主从复制**: 支持读写分离的数据库复制架构
//!
//! ## 模块结构
//!
//! - `db`: 数据库 ORM 核心模块
//!   - `model`: ORM 模型定义和操作
//!   - `query`: 查询构建器
//!   - `transaction`: 事务管理
//!   - `database`: 数据库连接和连接池
//!   - `cache`: 查询结果缓存
//!   - `logger`: 查询日志记录
//!   - `pool_monitor`: 连接池监控
//!   - `replication`: 主从复制管理
//!   - `query_plan_cache`: 查询计划缓存
//! - `redis`: Redis 客户端和操作封装

pub mod db {
    pub mod model;
    pub mod query;
    pub mod transaction;
    pub mod database;
    pub mod cache;

    pub use model::*;
    pub use query::*;
    pub use transaction::*;
    pub use database::*;
    pub use cache::*;
}

pub mod redis;

pub use redis::RedisClient;

pub use db::*;
pub use redis::*;

