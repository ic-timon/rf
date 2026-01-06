//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # RF 文本处理模块
//!
//! 提供文本处理功能的模块。
//!
//! 本模块提供了多种文本处理工具，包括字符串操作和正则表达式处理。
//! 为了避免命名冲突，模块被组织成子模块的结构，用户可以通过模块路径访问具体的功能。
//!
//! ## 模块结构
//!
//! - [`regex`] - 提供正则表达式相关的功能，如模式匹配、查找、替换等
//! - [`str`] - 提供基础字符串操作功能，如分割、连接、大小写转换等
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_text::str::replace;
//! use rf_text::regex::is_match;
//!
//! // 字符串替换
//! let result = replace("hello world", "world", "rust");
//! assert_eq!(result, "hello rust");
//!
//! // 正则表达式匹配
//! let matched = is_match(r"\d+", "abc123def").unwrap();
//! assert!(matched);
//! ```
//!
//! ## 注意事项
//!
//! 为了避免不同子模块间的函数名冲突（例如 `regex::replace` 和 `str::replace`），
//! 本模块不使用 glob 重新导出。用户应通过完整的模块路径访问函数：
//! `rf_text::regex::replace` 或 `rf_text::str::replace`。

pub mod regex;
pub mod str;

// 注意：我们不使用 glob 重新导出以避免以下冲突：
// - regex::replace 和 str::replace
// 用户应该通过模块路径访问函数：text::regex::replace 等。

