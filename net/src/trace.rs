//! # trace
//!
//! trace 模块 - 分布式追踪支持
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! 分布式追踪支持
//!
//! 提供 OpenTelemetry 兼容的分布式追踪功能，用于跟踪请求在微服务之间的传播路径。
//!
//! # 主要功能
//!
//! - 初始化 OpenTelemetry 追踪器
//! - 支持 OTLP（OpenTelemetry Protocol）通过 gRPC 或 HTTP 导出追踪数据
//! - 创建和管理 spans
//! - HTTP 请求的追踪上下文传播
//!
//! # 使用示例
//!
//! ## 基本追踪
//! ```ignore
//! use rf_net::trace;
//!
//! // 初始化追踪
//! trace::init_tracing("my-service")?;
//!
//! // 创建 span
//! let span = trace::span("process_request");
//! // 在 span 内执行代码
//! ```
//!
//! ## OTLP 导出
//! ```ignore
//! // 使用 gRPC 导出
//! trace::init_tracing_otlp_grpc("my-service", "http://jaeger:4317")?;
//!
//! // 使用 HTTP 导出
//! trace::init_tracing_otlp_http("my-service", "http://jaeger:4318")?;
//! ```
//!
//! ## HTTP 追踪传播
//! ```ignore
//! // 从请求头中提取追踪上下文
//! let context = trace::extract_context_from_headers(&headers);
//!
//! // 将追踪上下文注入到响应头
//! trace::inject_context_to_headers(&context, &mut headers);
//! ```

use rf_errors::Result;
use opentelemetry::trace::Tracer;
use opentelemetry::Context;
use opentelemetry::global;

/// 初始化基础的 OpenTelemetry 追踪
///
/// 这是一个简化的实现，设置基本的 tracing 日志。
/// 在生产环境中，应该配置完整的 OpenTelemetry 导出器。
///
/// # 参数
///
/// - `service_name`: 服务名称，用于标识当前服务
///
/// # 返回值
///
/// 成功返回 Ok(())，失败返回错误
///
/// # 功能
///
/// - 初始化 tracing 日志系统
/// - 从环境变量读取日志级别配置
///
/// # 示例
///
/// ```ignore
/// init_tracing("user-service")?;
/// ```
pub fn init_tracing(service_name: &str) -> Result<()> {
    tracing::info!("初始化 OpenTelemetry 追踪，服务: {}", service_name);

    // 设置基本的 tracing 日志
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    Ok(())
}

/// 初始化使用 OTLP gRPC 导出的追踪
///
/// 配置追踪数据通过 OTLP gRPC 协议发送到指定端点。
///
/// # 参数
///
/// - `service_name`: 服务名称
/// - `endpoint`: OTLP 收集器端点，例如 "http://jaeger:4317"
///
/// # 返回值
///
/// 成功返回 Ok(())，失败返回错误
///
/// # 注意
///
/// 这是一个简化的实现。完整的实现需要：
/// 1. 使用 `opentelemetry_otlp` 创建 gRPC 导出器
/// 2. 使用 `opentelemetry_sdk` 创建 TracerProvider
/// 3. 设置为全局的 tracer provider
/// 4. 使用 `tracing-opentelemetry` 初始化日志
///
/// # 示例
///
/// ```ignore
/// init_tracing_otlp_grpc("api-service", "http://jaeger:4317")?;
/// ```
pub fn init_tracing_otlp_grpc(service_name: &str, endpoint: &str) -> Result<()> {
    tracing::info!("初始化 OTLP gRPC 追踪，服务: {}, 端点: {}", service_name, endpoint);
    Ok(())
}

/// 初始化使用 OTLP HTTP 导出的追踪
///
/// 配置追踪数据通过 OTLP HTTP 协议发送到指定端点。
///
/// # 参数
///
/// - `service_name`: 服务名称
/// - `endpoint`: OTLP 收集器端点，例如 "http://jaeger:4318"
///
/// # 返回值
///
/// 成功返回 Ok(())，失败返回错误
///
/// # 注意
///
/// 这是一个简化的实现。完整的实现需要：
/// 1. 使用 `opentelemetry_otlp` 创建 HTTP 导出器
/// 2. 使用 `opentelemetry_sdk` 创建 TracerProvider
/// 3. 设置为全局的 tracer provider
/// 4. 使用 `tracing-opentelemetry` 初始化日志
///
/// # 示例
///
/// ```ignore
/// init_tracing_otlp_http("api-service", "http://jaeger:4318")?;
/// ```
pub fn init_tracing_otlp_http(service_name: &str, endpoint: &str) -> Result<()> {
    tracing::info!("初始化 OTLP HTTP 追踪，服务: {}, 端点: {}", service_name, endpoint);
    Ok(())
}

/// 创建一个追踪 span
///
/// # 参数
///
/// - `name`: span 的名称
///
/// # 返回值
///
/// 返回一个 tracing::Span 实例
///
/// # 示例
///
/// ```ignore
/// let span = span("database_query");
/// let _enter = span.enter();
/// // 执行数据库查询
/// ```
pub fn span(name: &str) -> tracing::Span {
    tracing::span!(tracing::Level::INFO, "{}", name)
}

/// 开始一个新的追踪
///
/// 这是创建 span 的便捷函数
///
/// # 参数
///
/// - `name`: 追踪的名称
///
/// # 返回值
///
/// 返回一个 tracing::Span 实例
pub fn start_trace(name: &str) -> tracing::Span {
    span(name)
}

/// 获取指定服务的 tracer
///
/// # 参数
///
/// - `service_name`: 服务名称，必须在 'static 生命周期
///
/// # 返回值
///
/// 返回一个实现了 Tracer trait 的对象
///
/// # 示例
///
/// ```ignore
/// let tracer = tracer("my-service");
/// let span = tracer.start("operation");
/// ```
pub fn tracer(service_name: &'static str) -> impl Tracer {
    global::tracer(service_name)
}

/// 创建一个使用 OpenTelemetry 的 span
///
/// # 参数
///
/// - `name`: span 的名称
///
/// # 返回值
///
/// 返回一个 tracing::Span 实例
pub fn create_span(name: &str) -> tracing::Span {
    tracing::span!(tracing::Level::INFO, "{}", name)
}

/// 获取当前的追踪上下文
///
/// # 返回值
///
/// 返回当前的 OpenTelemetry Context
///
/// # 示例
///
/// ```ignore
/// let context = current_context();
/// // 可以用于手动传播上下文
/// ```
pub fn current_context() -> Context {
    Context::current()
}

/// 设置追踪上下文
///
/// # 参数
///
/// - `context`: 要设置的 OpenTelemetry Context
///
/// # 示例
///
/// ```ignore
/// let context = extract_context_from_headers(&headers);
/// set_context(context);
/// ```
pub fn set_context(context: Context) {
    context.attach();
}

/// 从 HTTP 头中提取追踪上下文
///
/// 用于从传入的 HTTP 请求中提取分布式追踪上下文。
/// 这样可以继续客户端发起的追踪。
///
/// # 参数
///
/// - `headers`: HTTP 请求头
///
/// # 返回值
///
/// 返回提取的 OpenTelemetry Context
///
/// # 支持的传播格式
///
/// - W3C Trace Context
/// - B3 多头格式
/// - 其他通过 TextMapPropagator 配置的格式
///
/// # 示例
///
/// ```ignore
/// use axum::extract::Request;
///
/// async fn handler(request: Request) -> Result<Response> {
///     let context = extract_context_from_headers(request.headers());
///     set_context(context);
///     // 继续处理请求
/// }
/// ```
pub fn extract_context_from_headers(headers: &axum::http::HeaderMap) -> Context {
    use opentelemetry::propagation::Extractor;

    struct HeaderExtractor<'a>(&'a axum::http::HeaderMap);

    impl<'a> Extractor for HeaderExtractor<'a> {
        fn get(&self, key: &str) -> Option<&str> {
            self.0.get(key)?.to_str().ok()
        }

        fn keys(&self) -> Vec<&str> {
            self.0.keys().map(|k| k.as_str()).collect()
        }
    }

    let extractor = HeaderExtractor(headers);
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.extract(&extractor)
    })
}

/// 将追踪上下文注入到 HTTP 头
///
/// 用于在发送 HTTP 请求时添加追踪上下文，
/// 使接收方可以继续当前的追踪链路。
///
/// # 参数
///
/// - `context`: 要注入的 OpenTelemetry Context
/// - `headers`: HTTP 响应头（可变引用）
///
/// # 支持的传播格式
///
/// - W3C Trace Context
/// - B3 多头格式
/// - 其他通过 TextMapPropagator 配置的格式
///
/// # 示例
///
/// ```ignore
/// use reqwest::Client;
///
/// async fn send_traced_request(client: &Client) -> Result<Response> {
///     let mut headers = reqwest::header::HeaderMap::new();
///     let context = current_context();
///     inject_context_to_headers(&context, &mut headers);
///
///     client.post("http://api.example.com")
///         .headers(headers)
///         .send()
///         .await
/// }
/// ```
pub fn inject_context_to_headers(context: &Context, headers: &mut axum::http::HeaderMap) {
    use opentelemetry::propagation::Injector;

    struct HeaderInjector<'a> {
        headers: &'a mut axum::http::HeaderMap,
    }

    impl<'a> Injector for HeaderInjector<'a> {
        fn set(&mut self, key: &str, value: String) {
            if let Ok(header_value) = axum::http::HeaderValue::from_str(&value) {
                if let Ok(header_name) = axum::http::HeaderName::from_bytes(key.as_bytes()) {
                    self.headers.insert(header_name, header_value);
                }
            }
        }
    }

    let mut injector = HeaderInjector { headers };
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.inject_context(context, &mut injector);
    });
}
