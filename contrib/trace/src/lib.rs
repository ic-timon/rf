//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! RF Distributed Tracing Module
//!
//! Provides distributed tracing support using OpenTelemetry.

pub mod otlp;

pub use otlp::*;

/// Initialize tracing with OpenTelemetry
pub fn init_tracing(service_name: &str) -> rf_errors::Result<()> {
    // Initialize OpenTelemetry tracer
    tracing::info!("Initializing OpenTelemetry tracing for service: {}", service_name);
    
    // Set up basic tracing subscriber if not already initialized
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
    
    Ok(())
}

/// Create a span for tracing
pub fn span(name: &str) -> tracing::Span {
    tracing::span!(tracing::Level::INFO, "{}", name)
}

/// Start a new trace
pub fn start_trace(name: &str) -> tracing::Span {
    span(name)
}

/// Get the current tracer
/// Note: The tracer uses the service name internally
pub fn tracer(service_name: &'static str) -> impl opentelemetry::trace::Tracer {
    opentelemetry::global::tracer(service_name)
}

/// Create a span using OpenTelemetry
pub fn create_span(name: &str) -> tracing::Span {
    tracing::span!(tracing::Level::INFO, "{}", name)
}

/// Get the current context
pub fn current_context() -> opentelemetry::Context {
    opentelemetry::Context::current()
}

/// Set context
pub fn set_context(context: opentelemetry::Context) {
    context.attach();
}

/// Extract trace context from headers (for HTTP propagation)
pub fn extract_context_from_headers(headers: &axum::http::HeaderMap) -> opentelemetry::Context {
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

/// Inject trace context into headers (for HTTP propagation)
pub fn inject_context_to_headers(context: &opentelemetry::Context, headers: &mut axum::http::HeaderMap) {
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

