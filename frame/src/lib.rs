//! # lib
//!
//! lib 模块 - RF 框架核心模块入口
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # RF Frame Module
//!
//! RF 框架核心模块，提供框架实例管理和便捷功能函数。
//!
//! ## 模块结构
//!
//! - **g 模块**: 提供全局便捷函数，用于快速创建各种服务实例（服务器、客户端、数据库等）
//! - **gins 模块**: 提供全局实例管理器，用于管理和复用框架中的各种实例
//!
//! ## 主要功能
//!
//! ### 便捷函数 (g 模块)
//! - HTTP/TCP/UDP 服务器和客户端
//! - 数据库连接和 ORM 模型
//! - Redis 缓存
//! - 配置管理
//! - 视图引擎
//! - 日志系统
//! - 国际化支持
//! - 资源管理
//! - 数据验证
//! - 异步任务执行
//! - 调试工具
//!
//! ### 实例管理 (gins 模块)
//! - 单例模式的实例管理
//! - 基于配置的实例创建
//! - 实例复用和共享
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_frame::{g, gins};
//!
//! // 使用便捷函数快速创建实例
//! let server = g::server("127.0.0.1:8080".parse().unwrap());
//! let client = g::client();
//!
//! // 使用实例管理器获取或创建实例
//! let db = gins::database(Some("default")).await?;
//! ```
//!

pub mod g;
pub mod gins;

// Re-export with specific names to avoid conflicts
// 重新导出并使用特定名称以避免命名冲突
pub use g::{
    server as g_server,
    client as g_client,
    db as g_db,
    model as g_model,
    redis as g_redis,
    config as g_config,
    view as g_view,
    log as g_log,
    i18n as g_i18n,
    resource as g_resource,
    validator as g_validator,
    go as g_go,
    wait as g_wait,
    listen as g_listen,
    dump as g_dump,
    dump_with_type as g_dump_with_type,
    r#try as g_try,
    try_catch as g_try_catch,
    is_nil as g_is_nil,
    is_empty as g_is_empty,
};
pub use gins::{
    server,
    database,
    redis,
    view,
    config,
    i18n,
    resource,
};

