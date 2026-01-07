//! # oai
//!
//! oai 模块 - OpenAPI 规范支持
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! OpenAPI 支持模块
//!
//! 提供 OpenAPI 规范（Swagger）的构建功能，用于自动生成 API 文档。
//!
//! # 主要功能
//!
//! - 创建 OpenAPI 规范文档
//! - 配置 API 信息（标题、版本、描述）
//! - 生成标准化的 API 文档
//!
//! # 使用示例
//!
//! ## 基本使用
//! ```ignore
//! use rf_net::oai::OpenApiBuilder;
//!
//! let openapi = OpenApiBuilder::new("My API", "1.0.0")
//!     .description("这是一个示例 API")
//!     .build();
//! ```
//!
//! ## 与 HTTP 服务器集成
//! ```ignore
//! use rf_net::http::HttpServer;
//!
//! let server = HttpServer::new(addr)
//!     .with_swagger_ui(openapi, "/api-docs");
//! ```
//!
//! # 注意
//!
//! 这是一个简化的实现。对于完整的 OpenAPI 支持，建议使用 utoipa 库的派生宏。

/// OpenAPI 规范构建器
///
/// 用于构建 OpenAPI (Swagger) 规范文档，描述 API 的接口、参数、响应等。
///
/// # 字段
///
/// - `title`: API 标题
/// - `version`: API 版本号
/// - `description`: API 描述（可选）
///
/// # 示例
///
/// ```ignore
/// let builder = OpenApiBuilder::new("User API", "2.0.0")
///     .description("用户管理 API");
///
/// let spec = builder.build();
/// ```
pub struct OpenApiBuilder {
    /// API 标题
    title: String,
    /// API 版本
    version: String,
    /// API 描述（可选）
    description: Option<String>,
}

impl OpenApiBuilder {
    /// 创建一个新的 OpenAPI 构建器
    ///
    /// # 参数
    ///
    /// - `title`: API 的标题，例如 "用户管理 API"
    /// - `version`: API 的版本号，例如 "1.0.0"
    ///
    /// # 返回值
    ///
    /// 返回一个 OpenApiBuilder 实例
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let builder = OpenApiBuilder::new("My API", "1.0.0");
    /// ```
    pub fn new(title: &str, version: &str) -> Self {
        Self {
            title: title.to_string(),
            version: version.to_string(),
            description: None,
        }
    }

    /// 设置 API 描述
    ///
    /// # 参数
    ///
    /// - `description`: API 的详细描述
    ///
    /// # 返回值
    ///
    /// 返回修改后的 OpenApiBuilder 实例（支持链式调用）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let builder = OpenApiBuilder::new("My API", "1.0.0")
    ///     .description("这是一个 RESTful API 服务");
    /// ```
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// 构建 OpenAPI 规范
    ///
    /// # 返回值
    ///
    /// 返回一个完整的 OpenAPI 规范对象
    ///
    /// # 注意
    ///
    /// 这是一个简化的实现，只包含基本信息。
    /// 对于完整的 OpenAPI 支持，应该使用 utoipa 的派生宏：
    ///
    /// ```ignore
    /// #[derive(OpenApi)]
    /// #[openapi(paths(...), components(...))]
    /// struct ApiDoc;
    /// ```
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let spec = builder.build();
    /// let json = serde_json::to_string(&spec)?;
    /// ```
    pub fn build(self) -> utoipa::openapi::OpenApi {
        // 创建基本的 OpenAPI 规范
        let mut spec = utoipa::openapi::OpenApi::default();
        let info = utoipa::openapi::InfoBuilder::new()
            .title(&self.title)
            .version(&self.version);
        let info = if let Some(desc) = &self.description {
            info.description(Some(desc))
        } else {
            info
        };
        spec.info = info.build();
        spec
    }
}

/// 创建默认的 OpenAPI 规范
///
/// 创建一个预配置的 OpenAPI 规范，包含基本的 RF 框架 API 信息。
///
/// # 返回值
    /// 返回一个默认的 OpenAPI 规范对象
///
/// # 示例
///
/// ```ignore
/// let openapi = default_openapi();
/// // 在 HTTP 服务器中使用
/// server.with_swagger_ui(openapi, "/docs");
/// ```
pub fn default_openapi() -> utoipa::openapi::OpenApi {
    OpenApiBuilder::new("RF API", "1.0.0")
        .description("RF Framework API Documentation")
        .build()
}
