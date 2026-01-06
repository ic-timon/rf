//! # lib
//!
//! lib 模块 - RF 框架错误处理模块的入口文件
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # RF 框架错误处理模块
//!
//! 本模块为 RF 框架提供统一的错误码定义和错误处理机制。
//!
//! ## 主要功能
//!
//! - **错误码定义**：提供标准化的错误码常量定义
//! - **错误类型**：定义框架内使用的所有错误类型
//! - **错误转换**：支持与其他错误类型的自动转换
//!
//! ## 模块结构
//!
//! - [`code`]: 错误码定义模块，包含所有标准错误码常量
//! - [`error`]: 错误类型定义模块，包含错误枚举和处理逻辑
//!
//! ## 使用示例
//!
//! ```rust
//! use errors::{RfError, Result, codes};
//!
//! fn example_function() -> Result<String> {
//!     Err(RfError::NotFound("资源未找到".to_string()))
//! }
//!
//! fn main() {
//!     match example_function() {
//!         Ok(data) => println!("{}", data),
//!         Err(e) => {
//!             eprintln!("错误码: {}", e.code());
//!             eprintln!("错误信息: {}", e.message());
//!         }
//!     }
//! }
//! ```

pub mod code;
pub mod error;

// 重新导出公共 API，方便外部使用
pub use code::*;
pub use error::*;

