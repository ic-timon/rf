//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # RF 测试模块
//!
//! 提供测试工具和辅助函数。
//!
//! ## 模块说明
//!
//! 该模块是 RF 框架的测试工具库，提供了常用的测试辅助函数和断言工具。
//!
//! ## 子模块
//!
//! - [`test`]: 提供断言和测试辅助函数
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_test::assert_eq;
//! use rf_test::assert;
//!
//! fn example() {
//!     assert_eq!(1 + 1, 2);
//!     assert!(true);
//! }
//! ```

pub mod test;

pub use test::*;

