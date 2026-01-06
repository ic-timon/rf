//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! RF gRPC framework
//!
//! Provides gRPC server and client encapsulation with middleware support

pub mod server;
pub mod client;
pub mod middleware;

pub use server::GrpcServer;
pub use client::GrpcClient;
pub use middleware::*;

