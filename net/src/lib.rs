//! # lib
//!
//! lib 模块 - RF 网络模块的根模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! RF 网络模块
//!
//! 该模块提供了完整的网络功能支持，包括：
//! - HTTP 服务器和客户端
//! - TCP 和 UDP 网络协议
//! - IPv4 和 IPv6 地址处理
//! - 负载均衡和服务发现
//! - 分布式追踪支持
//! - OpenAPI 文档生成
//!
//! # 主要功能
//!
//! ## HTTP 模块
//! 提供 HTTP 服务器、客户端、路由、中间件、WebSocket、文件上传等功能
//!
//! ## 网络协议
//! - TCP：可靠的传输层协议
//! - UDP：无连接的传输层协议
//! - IPv4/IPv6：互联网协议版本 4 和 6
//!
//! ## 高级功能
//! - 负载均衡：支持轮询、随机、加权、最少连接等策略
//! - 服务发现：服务注册和发现机制
//! - 分布式追踪：基于 OpenTelemetry 的链路追踪
//! - API 文档：自动生成 OpenAPI 规范和 Swagger UI

pub mod http {
    pub mod middleware;
    pub mod request;
    pub mod response;
    pub mod server;
    pub mod hooks;
    pub mod websocket;
    pub mod static_files;
    pub mod router;
    pub mod rate_limit;
    pub mod timeout;
    pub mod upload;
    pub mod swagger;
    
    pub use middleware::*;
    pub use request::*;
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
}
pub mod client;
pub mod tcp;
pub mod udp;
pub mod ipv4;
pub mod ipv6;
pub mod sel;
pub mod svc;
pub mod trace;
pub mod oai;

pub use http::*;
pub use client::*;
pub use tcp::*;
pub use udp::*;
// Re-export with specific names to avoid conflicts
pub use ipv4::{parse as ipv4_parse, in_network as ipv4_in_network};
pub use ipv6::{parse as ipv6_parse, in_network as ipv6_in_network};
pub use sel::*;
pub use svc::*;
pub use trace::*;
pub use oai::*;
