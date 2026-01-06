//! # lib
//!
//! lib 模块 - RF 框架调试工具库的根模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! RF Debug Module - 调试工具模块
//!
//! 本模块提供了 RF 框架的调试工具集，包括：
//! - 堆栈跟踪获取
//! - 版本信息查询
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_debug::stack_trace;
//! use rf_debug::version;
//!
//! // 获取当前堆栈跟踪
//! let trace = stack_trace();
//! println!("{}", trace);
//!
//! // 获取版本信息
//! let ver = version();
//! println!("版本: {}", ver);
//! ```

pub mod debug;

pub use debug::*;

