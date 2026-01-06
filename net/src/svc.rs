//! # svc
//!
//! svc 模块 - 服务发现
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 服务发现
//!
//! 提供简单的服务注册和发现功能，用于微服务架构中的服务管理。
//!
//! # 主要功能
//!
//! - 服务注册：将服务实例注册到注册表
//! - 服务发现：根据服务名称查找服务实例
//!
//! # 使用示例
//!
//! ```ignore
//! use rf_net::svc::{Service, ServiceRegistry};
//!
//! // 创建服务注册表
//! let mut registry = ServiceRegistry::new();
//!
//! // 注册服务实例
//! let service = Service {
//!     name: "user-service".to_string(),
//!     address: "192.168.1.100".to_string(),
//!     port: 8080,
//! };
//! registry.register(service);
//!
//! // 发现服务
//! let instances = registry.discover("user-service");
//! for instance in instances {
//!     println!("服务实例: {}:{}", instance.address, instance.port);
//! }
//! ```

//! // 服务信息
//! //
//! // 表示一个服务实例的元数据
//! //
//! // # 字段
//! //
//! // - `name`: 服务名称，用于服务发现
//! // - `address`: 服务监听地址，例如 "192.168.1.100"
//! // - `port`: 服务监听端口
#[derive(Debug, Clone)]
pub struct Service {
    /// 服务名称
    pub name: String,
    /// 服务地址
    pub address: String,
    /// 服务端口
    pub port: u16,
}

/// 服务注册表
///
/// 管理所有已注册的服务实例
///
/// # 功能
///
/// - 注册新服务
/// - 根据名称查找服务
/// - 支持多个同名服务实例（用于负载均衡）
///
/// # 示例
///
/// ```ignore
/// let mut registry = ServiceRegistry::new();
///
/// // 注册多个服务实例
/// registry.register(Service {
///     name: "api-service".to_string(),
///     address: "192.168.1.10".to_string(),
///     port: 8080,
/// });
///
/// registry.register(Service {
///     name: "api-service".to_string(),
///     address: "192.168.1.11".to_string(),
///     port: 8080,
/// });
///
/// // 查找所有实例
/// let instances = registry.discover("api-service");
/// assert_eq!(instances.len(), 2);
/// ```
pub struct ServiceRegistry {
    /// 已注册的服务列表
    services: Vec<Service>,
}

impl ServiceRegistry {
    /// 创建一个新的服务注册表
    ///
    /// # 返回值
    ///
    /// 返回一个空的服务注册表
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let registry = ServiceRegistry::new();
    /// ```
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    /// 注册一个服务实例
    ///
    /// # 参数
    ///
    /// - `service`: 要注册的服务实例
    ///
    /// # 注意
    ///
    /// - 允许注册多个同名的服务实例（用于负载均衡）
    /// - 不会检查重复的实例
    ///
    /// # 示例
    ///
    /// ```ignore
    /// registry.register(Service {
    ///     name: "my-service".to_string(),
    ///     address: "127.0.0.1".to_string(),
    ///     port: 8080,
    /// });
    /// ```
    pub fn register(&mut self, service: Service) {
        self.services.push(service);
    }

    /// 根据服务名称查找服务实例
    ///
    /// # 参数
    ///
    /// - `name`: 要查找的服务名称
    ///
    /// # 返回值
    ///
    /// 返回所有匹配的服务实例。如果找不到，返回空列表。
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // 查找服务实例
    /// let instances = registry.discover("user-service");
    ///
    /// // 使用第一个实例
    /// if let Some(instance) = instances.first() {
    ///     let url = format!("http://{}:{}", instance.address, instance.port);
    /// }
    /// ```
    pub fn discover(&self, name: &str) -> Vec<&Service> {
        self.services.iter()
            .filter(|s| s.name == name)
            .collect()
    }
}

impl Default for ServiceRegistry {
    /// 创建默认的服务注册表
    fn default() -> Self {
        Self::new()
    }
}
