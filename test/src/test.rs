//! # test
//!
//! test 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 测试工具模块
//!
//! 提供基础的测试断言和辅助函数。
//!
//! ## 功能说明
//!
//! 该模块提供了两个常用的测试断言函数：
//! - [`assert_eq`]: 断言两个值相等
//! - [`assert`]: 断言条件为真
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_test::assert_eq;
//! use rf_test::assert;
//!
//! #[test]
//! fn test_example() {
//!     // 断言两个值相等
//!     assert_eq!(2 + 2, 4);
//!     assert_eq!("hello", "hello");
//!
//!     // 断言条件为真
//!     assert!(1 < 2);
//!     assert!(true);
//! }
//! ```

//! // 断言两个值相等
//! //
//! // 该函数用于在测试中验证两个值是否相等。如果两个值不相等，则会引发 panic。
//! //
//! // ## 泛型参数
//! //
//! // - `T`: 要比较的值类型，必须实现 `PartialEq` 和 `Debug` trait
//! //
//! // ## 参数
//! //
//! // - `left`: 左侧的期望值
//! // - `right`: 右侧的实际值
//! //
//! // ## Panics
//! //
//! // 当 `left` 和 `right` 不相等时，该函数会 panic 并显示两者的调试信息。
//! //
//! // ## 使用示例
//! //
//! // ```rust
//! // use rf_test::assert_eq;
//! //
//! // #[test]
//! // fn test_addition() {
//! //     assert_eq!(2 + 2, 4);  // 通过
//! //     assert_eq!(3 * 3, 9);  // 通过
//! // }
//! //
//! // #[test]
//! // #[should_panic]
//! // fn test_failure() {
//! //     assert_eq!(1, 2);  // 会 panic
//! // }
//! // ```
pub fn assert_eq<T: PartialEq + std::fmt::Debug>(left: T, right: T) {
    assert_eq!(left, right);
}

/// 断言条件为真
///
/// 该函数用于在测试中验证某个布尔条件是否为真。如果条件为假，则会引发 panic。
///
/// ## 参数
///
/// - `condition`: 要验证的布尔条件
///
/// ## Panics
///
/// 当 `condition` 为 `false` 时，该函数会 panic。
///
/// ## 使用示例
///
/// ```rust
/// use rf_test::assert;
///
/// #[test]
/// fn test_conditions() {
///     assert!(1 < 2);           // 通过
///     assert!(true);            // 通过
///     assert!(!false);          // 通过
/// }
///
/// #[test]
/// #[should_panic]
/// fn test_failure() {
///     assert!(false);  // 会 panic
/// }
/// ```
pub fn assert(condition: bool) {
    assert!(condition);
}

