//! # client
//!
//! client 模块 - HTTP 客户端
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP 客户端
//!
//! 提供了链式调用的 HTTP 客户端封装，支持发送各种类型的 HTTP 请求。
//! 基于 reqwest 库实现，提供了简洁的 API 来发送请求和处理响应。
//!
//! # 主要功能
//!
//! - 支持 GET、POST 等常见的 HTTP 方法
//! - 支持 JSON 请求体和响应
//! - 支持自定义请求头
//! - 支持文本响应
//! - 链式调用 API
//!
//! # 使用示例
//!
//! ```ignore
//! use rf_net::HttpClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // GET 请求获取 JSON
//!     let client = HttpClient::new();
//!     let data: serde_json::Value = client.get("https://api.example.com/data")
//!         .json().await?;
//!
//!     // POST 请求发送 JSON
//!     let response = client.post("https://api.example.com/create")
//!         .json_body(&serde_json::json!({"name": "test"}))
//!         .header("Authorization", "Bearer token")
//!         .send().await?;
//!
//!     Ok(())
//! }
//! ```

use reqwest::{Client, RequestBuilder, Response};
use rf_errors::{Result, RfError};
use serde::{Deserialize, Serialize};

/// HTTP 客户端封装
///
/// 提供了链式调用的 HTTP 客户端，用于发送 HTTP 请求并处理响应。
///
/// # 字段
///
/// - `client`: 底层的 reqwest Client 实例
/// - `builder`: 可选的请求构建器，用于链式调用
pub struct HttpClient {
    client: Client,
    builder: Option<RequestBuilder>,
}

impl HttpClient {
    /// 创建一个新的 HTTP 客户端
    ///
    /// # 返回值
    ///
    /// 返回一个初始化的 HttpClient 实例
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let client = HttpClient::new();
    /// ```
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            builder: None,
        }
    }

    /// 创建一个 GET 请求
    ///
    /// # 参数
    ///
    /// - `url`: 请求的目标 URL
    ///
    /// # 返回值
    ///
    /// 返回配置了 GET 方法的 HttpClient 实例，可以继续链式调用
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let client = HttpClient::new();
    /// let response = client.get("https://api.example.com/data")
    ///     .send().await?;
    /// ```
    pub fn get(&self, url: &str) -> Self {
        Self {
            client: self.client.clone(),
            builder: Some(self.client.get(url)),
        }
    }

    /// 创建一个 POST 请求
    ///
    /// # 参数
    ///
    /// - `url`: 请求的目标 URL
    ///
    /// # 返回值
    ///
    /// 返回配置了 POST 方法的 HttpClient 实例，可以继续链式调用
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let client = HttpClient::new();
    /// let response = client.post("https://api.example.com/create")
    ///     .json_body(&data)
    ///     .send().await?;
    /// ```
    pub fn post(&self, url: &str) -> Self {
        Self {
            client: self.client.clone(),
            builder: Some(self.client.post(url)),
        }
    }

    /// 设置 JSON 请求体
    ///
    /// # 参数
    ///
    /// - `body`: 要序列化为 JSON 的数据，必须实现 Serialize trait
    ///
    /// # 返回值
    ///
    /// 返回配置了 JSON 请求体的 HttpClient 实例
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let data = serde_json::json!({"name": "test", "value": 123});
    /// let response = client.post("https://api.example.com/create")
    ///     .json_body(&data)
    ///     .send().await?;
    /// ```
    pub fn json_body<T: Serialize>(mut self, body: &T) -> Self {
        if let Some(builder) = self.builder.take() {
            self.builder = Some(builder.json(body));
        }
        self
    }

    /// 设置请求头
    ///
    /// # 参数
    ///
    /// - `key`: 请求头名称
    /// - `value`: 请求头值
    ///
    /// # 返回值
    ///
    /// 返回配置了指定请求头的 HttpClient 实例
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.get("https://api.example.com/data")
    ///     .header("Authorization", "Bearer token")
    ///     .header("Accept", "application/json")
    ///     .send().await?;
    /// ```
    pub fn header(mut self, key: &str, value: &str) -> Self {
        if let Some(builder) = self.builder.take() {
            self.builder = Some(builder.header(key, value));
        }
        self
    }

    /// 发送 HTTP 请求
    ///
    /// # 返回值
    ///
    /// 返回 HTTP 响应对象，包含响应状态、头和体
    ///
    /// # 错误
    ///
    /// 如果网络请求失败，返回 RfError::Network 错误
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let response = client.get("https://api.example.com/data")
    ///     .send().await?;
    /// println!("Status: {}", response.status());
    /// ```
    pub async fn send(self) -> Result<Response> {
        let builder = self.builder.ok_or_else(|| RfError::Internal("No request builder".to_string()))?;
        builder.send().await
            .map_err(|e| RfError::Network(format!("Request failed: {}", e)))
    }

    /// 发送请求并解析 JSON 响应
    ///
    /// # 类型参数
    ///
    /// - `T`: 要反序列化的目标类型，必须实现 Deserialize trait
    ///
    /// # 返回值
    ///
    /// 返回反序列化后的 JSON 数据
    ///
    /// # 错误
    ///
    /// - 网络请求失败时返回 RfError::Network
    /// - JSON 解析失败时返回 RfError::Network
    ///
    /// # 示例
    ///
    /// ```ignore
    /// #[derive(Deserialize)]
    /// struct UserData {
    ///     id: u32,
    ///     name: String,
    /// }
    ///
    /// let user: UserData = client.get("https://api.example.com/user/1")
    ///     .json().await?;
    /// ```
    pub async fn json<T: for<'de> Deserialize<'de>>(self) -> Result<T> {
        let response = self.send().await?;
        response.json().await
            .map_err(|e| RfError::Network(format!("Failed to parse JSON: {}", e)))
    }

    /// 发送请求并获取文本响应
    ///
    /// # 返回值
    ///
    /// 返回响应体的文本内容
    ///
    /// # 错误
    ///
    /// - 网络请求失败时返回 RfError::Network
    /// - 文本读取失败时返回 RfError::Network
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let html = client.get("https://example.com")
    ///     .text().await?;
    /// println!("HTML content: {}", html);
    /// ```
    pub async fn text(self) -> Result<String> {
        let response = self.send().await?;
        response.text().await
            .map_err(|e| RfError::Network(format!("Failed to get text: {}", e)))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

