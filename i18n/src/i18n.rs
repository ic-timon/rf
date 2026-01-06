//! # i18n
//!
//! i18n 模块 - RF 框架国际化核心功能实现
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Internationalization (i18n) support
//!
//! // # 国际化（i18n）核心模块
//! //
//! // 本模块提供了完整的国际化支持，包括：
//! // - 多语言翻译数据管理
//! // - 线程安全的异步翻译接口
//! // - 动态语言环境切换
//! // - 参数化翻译支持
//! //
//! // ## 设计理念
//! //
//! // - 使用 `Arc<RwLock>` 实现线程安全的翻译数据共享
//! // - 异步 API 设计，便于在 Tokio 运行时中使用
//! // - 简单的键值对翻译映射
//! // - 降级机制：翻译缺失时返回原始键名
//! //
//! // ## 使用示例
//! //
//! // ```rust
//! // use rf_i18n::I18n;
//! // use std::collections::HashMap;
//! //
//! // #[tokio::main]
//! // async fn main() {
//! //     // 创建 i18n 管理器
//! //     let mut i18n = I18n::new("zh-CN");
//! //
//! //     // 加载中文翻译
//! //     let mut zh_translations = HashMap::new();
//! //     zh_translations.insert("welcome".to_string(), "欢迎".to_string());
//! //     zh_translations.insert("goodbye".to_string(), "再见".to_string());
//! //     i18n.load("zh-CN", zh_translations).await;
//! //
//! //     // 加载英文翻译
//! //     let mut en_translations = HashMap::new();
//! //     en_translations.insert("welcome".to_string(), "Welcome".to_string());
//! //     en_translations.insert("goodbye".to_string(), "Goodbye".to_string());
//! //     i18n.load("en", en_translations).await;
//! //
//! //     // 使用当前语言环境翻译
//! //     let text = i18n.translate("welcome").await;
//! //     println!("{}", text); // 输出: 欢迎
//! //
//! //     // 使用指定语言环境翻译
//! //     let text_en = i18n.translate_with_locale("en", "welcome").await;
//! //     println!("{}", text_en); // 输出: Welcome
//! //
//! //     // 带参数的翻译
//! //     let mut param_translations = HashMap::new();
//! //     param_translations.insert("greeting".to_string(), "你好, {name}!".to_string());
//! //     i18n.load("zh-CN", param_translations).await;
//! //
//! //     let mut params = HashMap::new();
//! //     params.insert("name".to_string(), "张三".to_string());
//! //     let text = i18n.translate_with_params("greeting", &params).await;
//! //     println!("{}", text); // 输出: 你好, 张三!
//! //
//! //     // 切换语言环境
//! //     i18n.set_locale("en");
//! //     println!("{}", i18n.locale()); // 输出: en
//! // }
//! // ```

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 翻译数据类型
///
/// 用于存储某个语言环境的所有翻译条目，键为翻译标识，键为翻译文本。
///
/// # 类型别名
///
/// 实际上是 `HashMap<String, String>` 的类型别名，用于提高代码可读性。
///
/// # 字段说明
///
/// - 键（Key）: 翻译标识符，用于唯一标识一个翻译条目（如 "hello", "user.name"）
/// - 值（Value）: 对应的翻译文本（如 "你好", "用户名"）
///
/// # 示例
///
/// ```rust
/// use rf_i18n::Translations;
/// use std::collections::HashMap;
///
/// let mut translations: Translations = HashMap::new();
/// translations.insert("hello".to_string(), "你好".to_string());
/// translations.insert("goodbye".to_string(), "再见".to_string());
/// ```
pub type Translations = HashMap<String, String>;

/// 国际化管理器
///
/// 负责管理多个语言环境的翻译数据，并提供翻译查询接口。
///
/// # 字段说明
///
/// - `locale`: 当前默认的语言环境代码（如 "zh-CN", "en", "ja"）
/// - `translations`: 所有语言环境的翻译数据，使用 `Arc<RwLock>` 实现线程安全共享
///
/// # 线程安全
///
/// 该结构体可以安全地在多线程环境中使用，内部使用 `Arc` 和 `RwLock` 保证线程安全。
///
/// # 异步设计
///
/// 所有需要访问翻译数据的方法都是异步的，使用 `async/await` 语法。
///
/// # 示例
///
/// ```rust
/// use rf_i18n::I18n;
///
/// // 创建新的 i18n 管理器
/// let i18n = I18n::new("zh-CN");
///
/// // 或使用默认实现（默认语言为 "en"）
/// let i18n = I18n::default();
/// ```
pub struct I18n {
    /// 当前默认的语言环境代码
    locale: String,
    /// 所有语言环境的翻译数据，使用 Arc<RwLock> 实现线程安全
    translations: Arc<RwLock<HashMap<String, Translations>>>,
}

impl I18n {
    /// 创建一个新的国际化管理器
    ///
    /// 初始化一个 i18n 管理器实例，并设置默认的语言环境。
    ///
    /// # 参数
    ///
    /// * `locale` - 默认语言环境代码（如 "zh-CN", "en", "ja", "ko"）
    ///
    /// # 返回值
    ///
    /// 返回一个初始化好的 `I18n` 实例
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rf_i18n::I18n;
    ///
    /// // 创建中文环境的管理器
    /// let i18n = I18n::new("zh-CN");
    ///
    /// // 创建英文环境的管理器
    /// let i18n = I18n::new("en");
    /// ```
    pub fn new(locale: &str) -> Self {
        Self {
            locale: locale.to_string(),
            translations: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 设置默认的语言环境
    ///
    /// 更改管理器的默认语言环境，这将影响后续的 `translate()` 调用。
    ///
    /// # 参数
    ///
    /// * `locale` - 新的语言环境代码（如 "zh-CN", "en", "ja"）
    ///
    /// # 注意
    ///
    /// - 此方法会立即生效，影响后续所有使用默认语言环境的翻译操作
    /// - 此方法不会重新加载翻译数据，需要确保目标语言环境的翻译数据已加载
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rf_i18n::I18n;
    ///
    /// let mut i18n = I18n::new("zh-CN");
    /// i18n.set_locale("en");
    /// assert_eq!(i18n.locale(), "en");
    /// ```
    pub fn set_locale(&mut self, locale: &str) {
        self.locale = locale.to_string();
    }

    /// 获取当前默认的语言环境
    ///
    /// 返回当前设置的默认语言环境代码。
    ///
    /// # 返回值
    ///
    /// 返回当前默认语言环境的字符串切片（如 "zh-CN", "en"）
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rf_i18n::I18n;
    ///
    /// let i18n = I18n::new("zh-CN");
    /// println!("{}", i18n.locale()); // 输出: zh-CN
    /// ```
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// 为指定语言环境加载翻译数据
    ///
    /// 将一组翻译数据加载到管理器中，如果该语言环境已存在数据，则会被覆盖。
    ///
    /// # 参数
    ///
    /// * `locale` - 要加载的语言环境代码（如 "zh-CN", "en"）
    /// * `translations` - 翻译数据的 HashMap，键为翻译标识，值为翻译文本
    ///
    /// # 注意
    ///
    /// - 此方法是异步的，需要在异步上下文中调用
    /// - 如果该语言环境已存在数据，将会被完全覆盖
    /// - 多个线程可以同时加载不同语言环境的数据
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rf_i18n::I18n;
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let i18n = I18n::new("zh-CN");
    ///
    ///     let mut translations = HashMap::new();
    ///     translations.insert("hello".to_string(), "你好".to_string());
    ///     translations.insert("world".to_string(), "世界".to_string());
    ///
    ///     i18n.load("zh-CN", translations).await;
    /// }
    /// ```
    pub async fn load(&self, locale: &str, translations: Translations) {
        let mut trans = self.translations.write().await;
        trans.insert(locale.to_string(), translations);
    }

    /// 使用当前默认语言环境翻译指定键
    ///
    /// 根据当前设置的默认语言环境，查找并返回对应的翻译文本。
    /// 如果翻译不存在，则返回原始键名作为降级处理。
    ///
    /// # 参数
    ///
    /// * `key` - 翻译键（如 "hello", "user.name"）
    ///
    /// # 返回值
    ///
    /// 返回翻译后的文本。如果翻译不存在，返回原始键名。
    ///
    /// # 注意
    ///
    /// - 此方法是异步的，需要在异步上下文中调用
    /// - 使用的是当前默认语言环境（可通过 `locale()` 或 `set_locale()` 查询/设置）
    /// - 如果需要使用特定语言环境，请使用 `translate_with_locale()`
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rf_i18n::I18n;
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let i18n = I18n::new("zh-CN");
    ///
    ///     let mut translations = HashMap::new();
    ///     translations.insert("hello".to_string(), "你好".to_string());
    ///     i18n.load("zh-CN", translations).await;
    ///
    ///     let text = i18n.translate("hello").await;
    ///     println!("{}", text); // 输出: 你好
    ///
    ///     // 不存在的键返回原始键名
    ///     let missing = i18n.translate("missing_key").await;
    ///     println!("{}", missing); // 输出: missing_key
    /// }
    /// ```
    pub async fn translate(&self, key: &str) -> String {
        self.translate_with_locale(&self.locale, key).await
    }

    /// 使用指定的语言环境翻译指定键
    ///
    /// 根据指定的语言环境，查找并返回对应的翻译文本。
    /// 如果翻译不存在，则返回原始键名作为降级处理。
    ///
    /// # 参数
    ///
    /// * `locale` - 要使用的语言环境代码（如 "zh-CN", "en"）
    /// * `key` - 翻译键（如 "hello", "user.name"）
    ///
    /// # 返回值
    ///
    /// 返回翻译后的文本。如果翻译不存在，返回原始键名。
    ///
    /// # 注意
    ///
    /// - 此方法是异步的，需要在异步上下文中调用
    /// - 不会影响当前默认语言环境的设置
    /// - 适用于需要临时使用其他语言环境的场景
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rf_i18n::I18n;
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let i18n = I18n::new("zh-CN");
    ///
    ///     // 加载中文翻译
    ///     let mut zh_trans = HashMap::new();
    ///     zh_trans.insert("hello".to_string(), "你好".to_string());
    ///     i18n.load("zh-CN", zh_trans).await;
    ///
    ///     // 加载英文翻译
    ///     let mut en_trans = HashMap::new();
    ///     en_trans.insert("hello".to_string(), "Hello".to_string());
    ///     i18n.load("en", en_trans).await;
    ///
    ///     // 使用不同语言环境翻译
    ///     let zh_text = i18n.translate_with_locale("zh-CN", "hello").await;
    ///     let en_text = i18n.translate_with_locale("en", "hello").await;
    ///
    ///     println!("{}", zh_text); // 输出: 你好
    ///     println!("{}", en_text); // 输出: Hello
    /// }
    /// ```
    pub async fn translate_with_locale(&self, locale: &str, key: &str) -> String {
        let trans = self.translations.read().await;
        if let Some(locale_trans) = trans.get(locale) {
            if let Some(value) = locale_trans.get(key) {
                return value.clone();
            }
        }
        // 降级处理：如果找不到翻译，返回原始键名
        key.to_string()
    }

    /// 使用参数进行翻译
    ///
    /// 翻译指定键，并用提供的参数替换翻译文本中的占位符。
    /// 占位符格式为 `{参数名}`。
    ///
    /// # 参数
    ///
    /// * `key` - 翻译键（如 "greeting", "user.info"）
    /// * `params` - 参数 HashMap，键为参数名，值为替换内容
    ///
    /// # 返回值
    ///
    /// 返回替换参数后的翻译文本。如果翻译不存在，返回原始键名。
    ///
    /// # 注意
    ///
    /// - 此方法是异步的，需要在异步上下文中调用
    /// - 占位符格式为 `{参数名}`（如 `{name}`, `{age}`）
    /// - 参数替换是简单的字符串替换，按参数名字匹配
    /// - 使用当前默认语言环境进行翻译
    ///
    /// # 示例
    ///
    /// ```rust
    /// use rf_i18n::I18n;
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let i18n = I18n::new("zh-CN");
    ///
    ///     // 加载带参数的翻译
    ///     let mut translations = HashMap::new();
    ///     translations.insert("greeting".to_string(), "你好, {name}!".to_string());
    ///     translations.insert("user_info".to_string(), "用户 {name}, 年龄 {age}".to_string());
    ///     i18n.load("zh-CN", translations).await;
    ///
    ///     // 单参数翻译
    ///     let mut params = HashMap::new();
    ///     params.insert("name".to_string(), "张三".to_string());
    ///     let text = i18n.translate_with_params("greeting", &params).await;
    ///     println!("{}", text); // 输出: 你好, 张三!
    ///
    ///     // 多参数翻译
    ///     let mut params2 = HashMap::new();
    ///     params2.insert("name".to_string(), "李四".to_string());
    ///     params2.insert("age".to_string(), "25".to_string());
    ///     let text2 = i18n.translate_with_params("user_info", &params2).await;
    ///     println!("{}", text2); // 输出: 用户 李四, 年龄 25
    /// }
    /// ```
    pub async fn translate_with_params(&self, key: &str, params: &HashMap<String, String>) -> String {
        let mut text = self.translate(key).await;
        for (k, v) in params {
            text = text.replace(&format!("{{{}}}", k), v);
        }
        text
    }
}

/// 为 I18n 实现 Default trait
///
/// 提供默认实例化方式，默认语言环境为 "en"（英语）。
///
/// # 示例
///
/// ```rust
/// use rf_i18n::I18n;
///
/// // 使用 Default trait 创建
/// let i18n: I18n = I18n::default();
/// assert_eq!(i18n.locale(), "en");
///
/// // 使用 Default::default() 创建
/// let i18n2 = I18n::default();
/// assert_eq!(i18n2.locale(), "en");
/// ```
impl Default for I18n {
    fn default() -> Self {
        Self::new("en")
    }
}
