//! # request
//!
//! request 模块 - HTTP 请求封装
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP 请求封装
//!
//! 提供 HTTP 请求的封装，方便访问请求的各个部分。
//!
//! # 主要功能
//!
//! - 访问 HTTP 方法、URI、头
//! - 解析查询参数
//! - 提取单个查询参数
//!
//! # 使用示例
//!
//! ```ignore
//! use rf_net::http::Request;
//!
//! async fn handler(request: Request) -> Result<Response> {
//!     // 获取 HTTP 方法
//!     let method = request.method();
//!
//!     // 获取 URI
//!     let uri = request.uri();
//!
//!     // 获取查询参数
//!     let params: QueryParams = request.query().await?;
//!
//!     // 获取单个查询参数
//!     if let Some(id) = request.query_param("id") {
//!         println!("ID: {}", id);
//!     }
//!
//!     Ok(Response::text("Hello"))
//! }
//! ```

use axum::extract::Query;
use axum::http::HeaderMap;
use axum::http::Method;
use axum::http::Uri;
use rf_errors::Result;
use serde::de::DeserializeOwned;

/// HTTP 请求封装
///
/// 提供对 HTTP 请求各部分的便捷访问。
///
/// # 字段
///
/// - `inner`: 底层的 axum Request
pub struct Request {
    inner: axum::extract::Request,
}

impl Request {
    /// 创建一个新的请求封装
    ///
    /// # 参数
    ///
    /// - `request`: axum 的原始请求对象
    ///
    /// # 返回值
    ///
    /// 返回一个 Request 封装实例
    pub fn new(request: axum::extract::Request) -> Self {
        Self { inner: request }
    }

    /// 获取 HTTP 方法
    ///
    /// # 返回值
    ///
    /// 返回 HTTP 方法（GET、POST、PUT、DELETE 等）
    ///
    /// # 示例
    ///
    /// ```ignore
    /// match request.method() {
    ///     &Method::GET => println!("GET 请求"),
    ///     &Method::POST => println!("POST 请求"),
    ///     _ => println!("其他方法"),
    /// }
    /// ```
    pub fn method(&self) -> &Method {
        self.inner.method()
    }

    /// 获取 URI
    ///
    /// # 返回值
    ///
    /// 返回请求的 URI，包含路径和查询参数
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let uri = request.uri();
    /// println!("请求路径: {}", uri.path());
    /// println!("查询字符串: {:?}", uri.query());
    /// ```
    pub fn uri(&self) -> &Uri {
        self.inner.uri()
    }

    /// 获取所有请求头
    ///
    /// # 返回值
    ///
    /// 返回请求头的映射表
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let headers = request.headers();
    /// if let Some(user_agent) = headers.get("user-agent") {
    ///     println!("User-Agent: {:?}", user_agent);
    /// }
    /// ```
    pub fn headers(&self) -> &HeaderMap {
        self.inner.headers()
    }

    /// 获取指定名称的请求头
    ///
    /// # 参数
    ///
    /// - `name`: 请求头名称（不区分大小写）
    ///
    /// # 返回值
    ///
    /// - `Some(&HeaderValue)`: 找到请求头
    /// - `None`: 请求头不存在
    ///
    /// # 示例
    ///
    /// ```ignore
    /// if let Some(auth) = request.header("authorization") {
    ///     println!("Authorization: {:?}", auth);
    /// }
    /// ```
    pub fn header(&self, name: &str) -> Option<&axum::http::HeaderValue> {
        self.headers().get(name)
    }

    /// 解析查询参数到结构体
    ///
    /// # 类型参数
    ///
    /// - `T`: 目标类型，必须实现 `DeserializeOwned`
    ///
    /// # 返回值
    ///
    /// 返回反序列化后的查询参数结构体
    ///
    /// # 错误
    ///
    /// 如果解析失败，返回 `RfError::Network`
    ///
    /// # 示例
    ///
    /// ```ignore
    /// #[derive(Deserialize)]
    /// struct QueryParams {
    ///     page: u32,
    ///     limit: u32,
    /// }
    ///
    /// let params: QueryParams = request.query().await?;
    /// println!("页码: {}, 每页: {}", params.page, params.limit);
    /// ```
    pub async fn query<T: DeserializeOwned>(&self) -> Result<T> {
        Query::try_from_uri(self.uri())
            .map(|q: Query<T>| q.0)
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to parse query: {}", e)))
    }

    /// 获取单个查询参数的值
    ///
    /// # 参数
    ///
    /// - `key`: 查询参数的名称
    ///
    /// # 返回值
    ///
    /// - `Some(String)`: 参数存在
    /// - `None`: 参数不存在
    ///
    /// # 示例
    ///
    /// ```ignore
    /// // URL: /search?q=rust&lang=cn
    /// if let Some(query) = request.query_param("q") {
    ///     println!("搜索关键词: {}", query);
    /// }
    /// ```
    pub fn query_param(&self, key: &str) -> Option<String> {
        self.uri().query().and_then(|q| {
            url::form_urlencoded::parse(q.as_bytes())
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.to_string())
        })
    }

    /// 获取原始的 axum 请求
    ///
    /// # 返回值
    ///
    /// 返回底层的 axum Request，用于高级用法
    pub fn into_inner(self) -> axum::extract::Request {
        self.inner
    }
}

impl From<axum::extract::Request> for Request {
    /// 从 axum Request 创建 Request
    fn from(request: axum::extract::Request) -> Self {
        Self::new(request)
    }
}
