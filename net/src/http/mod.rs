//! # mod
//!
//! mod 模块 - HTTP 服务器模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP 服务器模块
//!
//! 提供完整的 HTTP 服务器功能，包括路由、中间件、WebSocket、文件上传等。
//!
//! # 模块组织
//!
//! ## 核心模块
//! - `request`: HTTP 请求封装
//! - `response`: HTTP 响应封装
//! - `server`: HTTP 服务器实现
//! - `router`: 路由系统
//!
//! ## 中间件和增强
//! - `middleware`: 中间件系统
//! - `hooks`: 生命周期钩子
//! - `interceptor`: 请求/响应拦截器
//! - `plugin`: 插件系统
//!
//! ## 功能模块
//! - `websocket`: WebSocket 支持
//! - `static_files`: 静态文件服务
//! - `upload`: 文件上传处理
//! - `rate_limit`: 速率限制
//! - `timeout`: 请求超时
//! - `rewrite`: URL 重写
//! - `swagger`: Swagger UI 集成
//!
//! ## 管理模块
//! - `status_handler`: 状态码处理
//! - `server_manager`: 服务器实例管理

// 核心模块
pub mod middleware;
pub mod request;
pub mod response;
pub mod server;
pub mod hooks;
pub mod websocket;

// 功能模块
pub mod static_files;
pub mod router;
pub mod rate_limit;
pub mod timeout;
pub mod upload;
pub mod swagger;
pub mod rewrite;
pub mod status_handler;
pub mod server_manager;
pub mod interceptor;
pub mod plugin;

// 导出常用类型
pub use middleware::*;
pub use request::*;
pub use status_handler::*;
pub use server_manager::*;
pub use interceptor::*;
pub use plugin::*;
pub use response::*;
pub use server::*;
pub use hooks::*;
pub use websocket::*;
pub use static_files::*;
pub use router::*;
pub use rate_limit::*;
pub use timeout::*;
pub use upload::*;
pub use swagger::*;
pub use rewrite::*;
