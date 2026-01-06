//! # var 模块
//!
//! var 模块 - 通用变量类型
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # 通用变量类型
//!
//! 提供通用变量类型 (`Var`)，类似 GoFrame 的 `gvar.Var`。
//! // `Var` 可以存储任意类型的数据，并提供类型转换方法。
//! //
//! // # 特性
//! //
//! // - 动态类型：可以存储任意可序列化的类型
//! // - 类型转换：提供多种类型转换方法（string, int64, float64, bool）
//! // - 空值检查：支持检查值是否为空
//! // - 灵活构造：支持从多种类型直接构造
//! //
//! // # 示例
//!
//! ```
//! use rf_container::Var;
//!
//! // 从各种类型创建 Var
//! let var1 = Var::new("Hello");
//! let var2 = Var::new(42i64);
//! let var3 = Var::new(3.14f64);
//! let var4 = Var::new(true);
//!
//! // 类型转换
//! assert_eq!(var1.string(), "Hello");
//! assert_eq!(var2.int64(), 42);
//! assert_eq!(var3.float64(), 3.14);
//! assert_eq!(var4.bool(), true);
//! ```

use serde_json::Value;

/// 通用变量类型
///
/// 类似 GoFrame 的 `gvar.Var`，可以存储任意类型的数据。
/// 内部使用 `serde_json::Value` 实现，支持灵活的类型转换。
///
/// # 字段
///
/// - `0`: 内部的 `serde_json::Value`，存储实际数据
///
/// # 示例
///
/// ```
/// use rf_container::Var;
///
/// let var = Var::new("Hello, World!");
/// println!("{}", var.string());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Var(Value);

impl Var {
    /// 从任意可序列化的值创建一个新的 Var
    ///
    /// # 参数
    ///
    /// * `value`: 任意实现了 `serde::Serialize` 的值
    ///
    /// # 返回值
    ///
    /// 返回一个 `Var` 实例
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let var = Var::new(42);
    /// let var2 = Var::new("hello");
    /// let var3 = Var::new(vec![1, 2, 3]);
    /// ```
    pub fn new<T: serde::Serialize>(value: T) -> Self {
        Self(
            serde_json::to_value(value).unwrap_or(Value::Null)
        )
    }

    /// 获取内部的 JSON 值
    ///
    /// # 返回值
    ///
    /// 返回内部 `serde_json::Value` 的引用
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let var = Var::new(42);
    /// let inner = var.value();
    /// ```
    pub fn value(&self) -> &Value {
        &self.0
    }

    /// 转换为字符串
    ///
    /// # 返回值
    ///
    /// 返回字符串表示：
    /// - 如果是字符串类型，直接返回
    /// - 如果是数字类型，返回数字的字符串形式
    /// - 如果是布尔类型，返回 "true" 或 "false"
    /// - 如果是 Null，返回空字符串
    /// - 其他类型返回 JSON 字符串
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let var1 = Var::new("hello");
    /// assert_eq!(var1.string(), "hello");
    ///
    /// let var2 = Var::new(42);
    /// assert_eq!(var2.string(), "42");
    ///
    /// let var3 = Var::new(true);
    /// assert_eq!(var3.string(), "true");
    /// ```
    pub fn string(&self) -> String {
        match &self.0 {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => String::new(),
            _ => serde_json::to_string(&self.0).unwrap_or_default(),
        }
    }

    /// 转换为 i64 整数
    ///
    /// # 返回值
    ///
    /// 返回 i64 值：
    /// - 如果是数字类型，返回对应的 i64 值
    /// - 如果是字符串类型，尝试解析为 i64，失败返回 0
    /// - 如果是布尔类型，true 返回 1，false 返回 0
    /// - 其他类型返回 0
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let var1 = Var::new(42);
    /// assert_eq!(var1.int64(), 42);
    ///
    /// let var2 = Var::new("100");
    /// assert_eq!(var2.int64(), 100);
    ///
    /// let var3 = Var::new(true);
    /// assert_eq!(var3.int64(), 1);
    /// ```
    pub fn int64(&self) -> i64 {
        match &self.0 {
            Value::Number(n) => n.as_i64().unwrap_or(0),
            Value::String(s) => s.parse().unwrap_or(0),
            Value::Bool(b) => if *b { 1 } else { 0 },
            _ => 0,
        }
    }

    /// 转换为 f64 浮点数
    ///
    /// # 返回值
    ///
    /// 返回 f64 值：
    /// - 如果是数字类型，返回对应的 f64 值
    /// - 如果是字符串类型，尝试解析为 f64，失败返回 0.0
    /// - 其他类型返回 0.0
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let var1 = Var::new(3.14);
    /// assert_eq!(var1.float64(), 3.14);
    ///
    /// let var2 = Var::new("2.718");
    /// assert_eq!(var2.float64(), 2.718);
    /// ```
    pub fn float64(&self) -> f64 {
        match &self.0 {
            Value::Number(n) => n.as_f64().unwrap_or(0.0),
            Value::String(s) => s.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }

    /// 转换为布尔值
    ///
    /// # 返回值
    ///
    /// 返回布尔值：
    /// - 如果是布尔类型，直接返回对应值
    /// - 如果是数字类型，非零返回 true，零返回 false
    /// - 如果是字符串类型：
    ///   - 空字符串、 "0"、"false" 返回 false
    ///   - 其他字符串返回 true
    /// - 其他类型返回 false
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let var1 = Var::new(true);
    /// assert_eq!(var1.bool(), true);
    ///
    /// let var2 = Var::new(1);
    /// assert_eq!(var2.bool(), true);
    ///
    /// let var3 = Var::new(0);
    /// assert_eq!(var3.bool(), false);
    ///
    /// let var4 = Var::new("false");
    /// assert_eq!(var4.bool(), false);
    /// ```
    pub fn bool(&self) -> bool {
        match &self.0 {
            Value::Bool(b) => *b,
            Value::Number(n) => n.as_i64().unwrap_or(0) != 0,
            Value::String(s) => !s.is_empty() && s != "0" && s != "false",
            _ => false,
        }
    }

    /// 检查值是否为空
    ///
    /// # 返回值
    ///
    /// 返回是否为空：
    /// - Null 值返回 true
    /// - 空字符串返回 true
    /// - 空数组返回 true
    /// - 空对象返回 true
    /// - 其他情况返回 false
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let var1 = Var::new(());
    /// assert!(var1.is_empty());
    ///
    /// let var2 = Var::new("");
    /// assert!(var2.is_empty());
    ///
    /// let var3 = Var::new("hello");
    /// assert!(!var3.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        match &self.0 {
            Value::Null => true,
            Value::String(s) => s.is_empty(),
            Value::Array(a) => a.is_empty(),
            Value::Object(o) => o.is_empty(),
            _ => false,
        }
    }
}

impl From<Value> for Var {
    /// 从 `serde_json::Value` 创建 `Var`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    /// use serde_json::json;
    ///
    /// let value = json!(42);
    /// let var = Var::from(value);
    /// ```
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl From<String> for Var {
    /// 从 `String` 创建 `Var`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let s = "hello".to_string();
    /// let var = Var::from(s);
    /// ```
    fn from(value: String) -> Self {
        Self(Value::String(value))
    }
}

impl From<i64> for Var {
    /// 从 `i64` 创建 `Var`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let var = Var::from(42i64);
    /// ```
    fn from(value: i64) -> Self {
        Self(Value::Number(value.into()))
    }
}

impl From<f64> for Var {
    /// 从 `f64` 创建 `Var`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let var = Var::from(3.14f64);
    /// ```
    fn from(value: f64) -> Self {
        Self(Value::Number(serde_json::Number::from_f64(value).unwrap_or(0.into())))
    }
}

impl From<bool> for Var {
    /// 从 `bool` 创建 `Var`
    ///
    /// # 示例
    ///
    /// ```
    /// use rf_container::Var;
    ///
    /// let var = Var::from(true);
    /// ```
    fn from(value: bool) -> Self {
        Self(Value::Bool(value))
    }
}
