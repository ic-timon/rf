//! # cfg 模块
//!
//! 配置管理模块，提供灵活的配置读取、验证、加密和监控功能。
//!
//! ## 主要组件
//!
//! - **Config**: 新的配置管理器（基于适配器模式）
//! - **Cfg**: 传统配置管理器（使用 config crate）
//! - **adapter**: 配置适配器（文件、环境变量、内存等）
//! - **encryption**: 配置加密/解密
//! - **validation**: 配置验证
//! - **watcher**: 配置文件监控
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! ========== 子模块声明 ==========

pub mod adapter;
pub mod watcher;
pub mod validation;
pub mod encryption;

// 导出子模块的公共接口
pub use adapter::*;
pub use watcher::*;
pub use validation::*;
pub use encryption::*;

use rf_errors::Result;
use std::collections::HashMap;
use std::sync::Arc;

// ========== 新的配置管理器（基于适配器）==========

/// 配置管理器
///
/// 使用适配器模式提供灵活的配置管理，支持多种配置源、验证和加密。
///
/// # 功能特性
///
/// - 支持多个配置适配器（优先级顺序）
/// - 支持配置验证
/// - 支持配置加密/解密
/// - 自动合并多个配置源
///
/// # 字段
///
/// - `adapters`: 配置适配器列表（按优先级排序）
/// - `validator`: 配置验证器（可选）
/// - `encryption`: 配置加密器（可选）
///
/// # 示例
///
/// ```rust
/// use rf::os::cfg::Config;
/// use rf::os::cfg::MemoryConfigAdapter;
/// use std::sync::Arc;
///
/// let config = Config::new()
///     .adapter(Arc::new(MemoryConfigAdapter::new()))
///     .with_validator(ConfigValidator::new())
///     .with_encryption(Arc::new(NoOpEncryption));
/// ```
pub struct Config {
    adapters: Vec<Arc<dyn ConfigAdapter>>,
    validator: Option<ConfigValidator>,
    encryption: Option<Arc<dyn ConfigEncryption>>,
}

impl Config {
    /// 创建新的配置管理器
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `Config` 实例
    pub fn new() -> Self {
        Self {
            adapters: Vec::new(),
            validator: None,
            encryption: None,
        }
    }

    /// 添加配置适配器
    ///
    /// 配置适配器按添加顺序进行查询，后面的适配器会覆盖前面的配置。
    ///
    /// # 参数
    ///
    /// - `adapter`: 配置适配器实例
    ///
    /// # 返回值
    ///
    /// 返回 `self`，支持链式调用
    pub fn adapter(mut self, adapter: Arc<dyn ConfigAdapter>) -> Self {
        self.adapters.push(adapter);
        self
    }

    /// 设置配置验证器
    ///
    /// # 参数
    ///
    /// - `validator`: 配置验证器
    ///
    /// # 返回值
    ///
    /// 返回 `self`，支持链式调用
    pub fn with_validator(mut self, validator: ConfigValidator) -> Self {
        self.validator = Some(validator);
        self
    }

    /// 设置配置加密器
    ///
    /// # 参数
    ///
    /// - `encryption`: 配置加密器
    ///
    /// # 返回值
    ///
    /// 返回 `self`，支持链式调用
    pub fn with_encryption(mut self, encryption: Arc<dyn ConfigEncryption>) -> Self {
        self.encryption = Some(encryption);
        self
    }

    /// 获取配置值
    ///
    /// 按适配器顺序查询，找到第一个匹配的配置值。
    /// 如果启用了加密，会自动解密配置值。
    ///
    /// # 参数
    ///
    /// - `key`: 配置键
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Option<String>>`，如果配置存在则返回 Some，否则返回 None
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        // 按顺序尝试适配器
        for adapter in &self.adapters {
            if let Ok(Some(mut value)) = adapter.get(key) {
                // 如果启用了加密，解密配置值
                if let Some(ref encryption) = self.encryption {
                    value = encryption.decrypt(&value)?;
                }
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    /// 设置配置值
    ///
    /// 如果设置了验证器，会先验证配置值。
    /// 如果启用了加密，会自动加密配置值。
    /// 配置值会写入第一个可写的适配器。
    ///
    /// # 参数
    ///
    /// - `key`: 配置键
    /// - `value`: 配置值
    ///
    /// # 返回值
    ///
    /// 返回 `Result<()>`，表示操作成功或失败
    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        // 如果设置了验证器，验证配置值
        if let Some(ref validator) = self.validator {
            validator.validate(key, value)?;
        }

        // 如果启用了加密，加密配置值
        let final_value = if let Some(ref encryption) = self.encryption {
            encryption.encrypt(value)?
        } else {
            value.to_string()
        };

        // 写入第一个可写的适配器
        for adapter in &self.adapters {
            if adapter.set(key, &final_value).is_ok() {
                return Ok(());
            }
        }
        Err(rf_errors::RfError::Config("No writable adapter available".to_string()))
    }

    /// 获取所有配置
    ///
    /// 合并所有适配器的配置，后面的适配器会覆盖前面的。
    /// 如果启用了加密，会自动解密所有配置值。
    /// 如果设置了验证器，会验证所有配置值。
    ///
    /// # 返回值
    ///
    /// 返回 `Result<HashMap<String, String>>`，包含所有配置键值对
    pub fn all(&self) -> Result<HashMap<String, String>> {
        let mut result = HashMap::new();
        // 合并所有适配器的配置（后面的适配器覆盖前面的）
        for adapter in &self.adapters {
            if let Ok(all) = adapter.all() {
                for (k, v) in all {
                    result.insert(k, v);
                }
            }
        }

        // 如果启用了加密，解密配置
        if let Some(ref encryption) = self.encryption {
            result = decrypt_config(&result, encryption.as_ref())?;
        }

        // 如果设置了验证器，验证所有配置
        if let Some(ref validator) = self.validator {
            validator.validate_all(&result)?;
        }

        Ok(result)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

// ========== 传统配置管理器（使用 config crate）==========

/// 传统配置管理器
///
/// 使用 config crate 提供的配置管理功能。
///
/// # 功能特性
///
/// - 支持多种配置格式（TOML、JSON、YAML 等）
/// - 支持环境变量替换
/// - 支持嵌套配置
///
/// # 字段
///
/// - `config`: config::Config 实例
///
/// # 示例
///
/// ```rust
/// use rf::os::cfg::Cfg;
///
/// // 从文件加载配置
/// let config = Cfg::load_file("config.toml")?;
///
/// // 获取配置值
/// let port: u32 = config.get("server.port")?;
/// ```
pub struct Cfg {
    config: config::Config,
}

impl Cfg {
    /// 创建新的配置管理器
    ///
    /// # 返回值
    ///
    /// 返回一个空的 `Cfg` 实例
    pub fn new() -> Self {
        Self {
            config: ::config::Config::default(),
        }
    }

    /// 从文件加载配置
    ///
    /// 支持的格式：TOML、JSON、YAML、RON 等
    ///
    /// # 参数
    ///
    /// - `path`: 配置文件路径（不含扩展名）
    ///
    /// # 返回值
    ///
    /// 返回 `Result<Self>`，包含加载的配置
    ///
    /// # 示例
    ///
    /// ```rust
    /// let config = Cfg::load_file("config/settings")?;
    /// // 会尝试加载 config/settings.toml, config/settings.json 等
    /// ```
    pub fn load_file(path: &str) -> Result<Self> {
        let config = ::config::Config::builder()
            .add_source(::config::File::with_name(path))
            .build()
            .map_err(|e| rf_errors::RfError::Config(format!("Failed to load config: {}", e)))?;
        Ok(Self { config })
    }

    /// 获取配置值
    ///
    /// # 类型参数
    ///
    /// - `T`: 目标类型（必须实现 `DeserializeOwned`）
    ///
    /// # 参数
    ///
    /// - `key`: 配置键（支持点号分隔的嵌套键，如 "server.port"）
    ///
    /// # 返回值
    ///
    /// 返回 `Result<T>`，包含解析后的配置值
    ///
    /// # 示例
    ///
    /// ```rust
    /// let port: u32 = config.get("server.port")?;
    /// let host: String = config.get("server.host")?;
    /// ```
    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T> {
        self.config
            .get(key)
            .map_err(|e| rf_errors::RfError::Config(format!("Failed to get config key {}: {}", key, e)))
    }

    /// 获取配置值，如果不存在则返回默认值
    ///
    /// # 类型参数
    ///
    /// - `T`: 目标类型（必须实现 `DeserializeOwned`）
    ///
    /// # 参数
    ///
    /// - `key`: 配置键
    /// - `default`: 默认值
    ///
    /// # 返回值
    ///
    /// 返回配置值或默认值
    ///
    /// # 示例
    ///
    /// ```rust
    /// let port = config.get_or("server.port", 8080);
    /// ```
    pub fn get_or<T: serde::de::DeserializeOwned>(&self, key: &str, default: T) -> T {
        self.get(key).unwrap_or(default)
    }

    /// 设置配置值
    ///
    /// 注意：config crate 不支持在创建后修改配置值，
    /// 这是一个占位实现。
    ///
    /// # 参数
    ///
    /// - `key`: 配置键
    /// - `value`: 配置值（必须实现 `Serialize`）
    ///
    /// # 返回值
    ///
    /// 返回 `Result<()>`
    pub fn set(&mut self, key: &str, value: impl serde::Serialize) -> Result<()> {
        // 注意：config crate 不支持在创建后设置值
        // 这是一个占位实现
        let _ = (key, value);
        Ok(())
    }
}

impl Default for Cfg {
    fn default() -> Self {
        Self::new()
    }
}
