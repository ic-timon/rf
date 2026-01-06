//! # lib
//!
//! lib 模块 - RF 框架国际化模块的根模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! RF I18n Module
//!
//! // # RF 国际化模块
//! //
//! // 本模块为 RF 框架提供国际化（i18n）支持功能。
//! //
//! // ## 主要功能
//! //
//! // - 多语言翻译管理
//! // - 动态语言切换
//! // - 参数化翻译支持
//! // - 线程安全的异步翻译接口
//! //
//! // ## 模块结构
//! //
//! // - [`i18n`] - 核心国际化功能实现
//! //
//! // ## 使用示例
//! //
//! // ```rust
//! // use rf_i18n::I18n;
//! //
//! // #[tokio::main]
//! // async fn main() {
//! //     let mut i18n = I18n::new("zh-CN");
//! //
//! //     // 加载翻译
//! //     let mut translations = std::collections::HashMap::new();
//! //     translations.insert("hello".to_string(), "你好".to_string());
//! //     i18n.load("zh-CN", translations).await;
//! //
//! //     // 使用翻译
//! //     let text = i18n.translate("hello").await;
//! //     println!("{}", text); // 输出: 你好
//! // }
//! // ```

pub mod i18n;

pub use i18n::*;

