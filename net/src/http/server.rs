//! # server
//!
//! server 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP server implementation

use axum::Router;
use rf_errors::Result;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use utoipa::openapi::OpenApi;
use std::sync::Arc;

/// Wrapper trait for service registry to make it object-safe
trait RegistryWrapper: Send + Sync {
    fn register(&self, instance: &rf_contrib_registry::ServiceInstance) -> rf_errors::Result<()>;
    fn deregister(&self, service_id: &str) -> rf_errors::Result<()>;
}

/// Implementation wrapper for ServiceRegistry
struct RegistryWrapperImpl<R: rf_contrib_registry::ServiceRegistry>(Arc<R>);

impl<R: rf_contrib_registry::ServiceRegistry> RegistryWrapper for RegistryWrapperImpl<R> {
    fn register(&self, instance: &rf_contrib_registry::ServiceInstance) -> rf_errors::Result<()> {
        rf_contrib_registry::ServiceRegistry::register(&*self.0, instance)
    }
    
    fn deregister(&self, service_id: &str) -> rf_errors::Result<()> {
        rf_contrib_registry::ServiceRegistry::deregister(&*self.0, service_id)
    }
}

/// HTTP server
pub struct HttpServer {
    router: Router,
    addr: SocketAddr,
    shutdown_timeout: Option<std::time::Duration>,
    max_request_body_size: Option<usize>,
    service_registry: Option<Box<dyn RegistryWrapper>>,
    service_name: Option<String>,
    service_id: Option<String>,
    health_check_path: Option<String>,
}

impl HttpServer {
    /// Create a new HTTP server
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            router: Router::new(),
            addr,
            shutdown_timeout: Some(std::time::Duration::from_secs(30)),
            max_request_body_size: None,
            service_registry: None,
            service_name: None,
            service_id: None,
            health_check_path: Some("/health".to_string()),
        }
    }

    /// Set service registry for automatic registration
    pub fn with_registry<R: rf_contrib_registry::ServiceRegistry + Send + Sync + 'static>(mut self, registry: Arc<R>, service_name: String, service_id: String) -> Self {
        self.service_registry = Some(Box::new(RegistryWrapperImpl(registry)));
        self.service_name = Some(service_name);
        self.service_id = Some(service_id);
        self
    }

    /// Set health check path
    pub fn health_check_path(mut self, path: String) -> Self {
        self.health_check_path = Some(path);
        self
    }

    /// Get the server address
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Set maximum request body size in bytes
    pub fn max_request_body_size(mut self, size: usize) -> Self {
        self.max_request_body_size = Some(size);
        self
    }

    /// Get the underlying router for route configuration
    pub fn router(&mut self) -> &mut Router {
        &mut self.router
    }

    /// Add logging middleware using tower-http
    pub fn with_logging(mut self) -> Self {
        self.router = self.router.layer(TraceLayer::new_for_http());
        self
    }

    /// Add CORS middleware using tower-http
    pub fn with_cors(mut self) -> Self {
        self.router = self.router.layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any),
        );
        self
    }

    /// Add response compression
    pub fn with_compression(mut self) -> Self {
        use tower_http::compression::CompressionLayer;
        self.router = self.router.layer(CompressionLayer::new());
        self
    }

    /// Add request timeout
    pub fn with_request_timeout(mut self, timeout: std::time::Duration) -> Self {
        use tower_http::timeout::TimeoutLayer;
        self.router = self.router.layer(TimeoutLayer::new(timeout));
        self
    }

    /// Add Swagger UI with OpenAPI specification
    pub fn with_swagger_ui(mut self, openapi: OpenApi, path: &str) -> Self {
        use super::swagger::create_swagger_ui_router;
        self.router = self.router.merge(create_swagger_ui_router(openapi, path));
        self
    }

    /// Set shutdown timeout
    pub fn shutdown_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.shutdown_timeout = Some(timeout);
        self
    }

    /// Start the server with graceful shutdown
    pub async fn serve(mut self) -> Result<()> {
        // Register with service registry if configured
        if let (Some(registry), Some(ref service_name), Some(ref service_id)) = 
            (self.service_registry.as_ref(), self.service_name.as_ref(), self.service_id.as_ref()) {
            let instance = rf_contrib_registry::ServiceInstance {
                id: (*service_id).clone(),
                name: (*service_name).clone(),
                address: self.addr,
                metadata: std::collections::HashMap::new(),
                health: rf_contrib_registry::ServiceHealth::Healthy,
            };
            registry.register(&instance)?;
            tracing::info!("Registered service {} ({}) with registry", service_name, service_id);
        }

        // Add health check endpoint if registry is configured
        if self.service_registry.is_some() {
            let health_path = self.health_check_path.clone().unwrap_or_else(|| "/health".to_string());
            self.router = self.router.route(&health_path, axum::routing::get(|| async {
                axum::Json(serde_json::json!({"status": "healthy"}))
            }));
        }

        let listener = TcpListener::bind(&self.addr).await
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to bind: {}", e)))?;
        
        tracing::info!("Server listening on {}", self.addr);
        
        // Create shutdown signal
        let registry_opt = self.service_registry.take();
        let service_id_clone = self.service_id.clone();
        let shutdown = async move {
            let ctrl_c = async {
                signal::ctrl_c()
                    .await
                    .expect("Failed to install Ctrl+C handler");
            };
            
            #[cfg(unix)]
            let terminate = async {
                signal::unix::signal(signal::unix::SignalKind::terminate())
                    .expect("Failed to install signal handler")
                    .recv()
                    .await;
            };
            
            #[cfg(not(unix))]
            let terminate = std::future::pending::<()>();
            
            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
            }
            
            tracing::info!("Shutdown signal received");
            
            // Deregister from service registry
            if let (Some(registry), Some(ref service_id)) = (registry_opt.as_ref(), service_id_clone.as_ref()) {
                if let Err(e) = registry.deregister(service_id) {
                    tracing::warn!("Failed to deregister service: {}", e);
                } else {
                    tracing::info!("Deregistered service {} from registry", service_id);
                }
            }
        };
        
        // Apply request body size limit if configured
        let router = if let Some(max_size) = self.max_request_body_size {
            self.router.layer(tower_http::limit::RequestBodyLimitLayer::new(max_size))
        } else {
            self.router
        };
        
        // Start server with graceful shutdown
        let server = axum::serve(listener, router)
            .with_graceful_shutdown(shutdown);
        
        if let Some(timeout) = self.shutdown_timeout {
            tokio::time::timeout(timeout, server)
                .await
                .map_err(|_| rf_errors::RfError::Network("Server shutdown timeout".to_string()))?
                .map_err(|e| rf_errors::RfError::Network(format!("Server error: {}", e)))?;
        } else {
            server.await
                .map_err(|e| rf_errors::RfError::Network(format!("Server error: {}", e)))?;
        }
        
        Ok(())
    }
}
