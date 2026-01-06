//! # lib
//!
//! lib 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Enhanced HTTP client SDK

use reqwest::Client;
use reqwest::RequestBuilder;
use rf_errors::Result;
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;

/// HTTP client with retry, load balancing, and circuit breaker
pub struct HttpClient {
    client: Client,
    base_url: Option<String>,
    retry_config: RetryConfig,
    load_balancer: Option<LoadBalancer>,
    circuit_breaker: Option<Arc<CircuitBreaker>>,
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub retry_on_status: Vec<u16>, // HTTP status codes to retry on
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            retry_on_status: vec![500, 502, 503, 504],
        }
    }
}

/// Load balancer for multiple endpoints
pub enum LoadBalanceStrategy {
    RoundRobin,
    Random,
    LeastConnections,
}

/// Load balancer
pub struct LoadBalancer {
    endpoints: Vec<String>,
    current_index: Arc<std::sync::atomic::AtomicUsize>,
    strategy: LoadBalanceStrategy,
}

impl LoadBalancer {
    pub fn new(endpoints: Vec<String>, strategy: LoadBalanceStrategy) -> Self {
        Self {
            endpoints,
            current_index: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            strategy,
        }
    }

    pub fn next(&self) -> Option<&String> {
        if self.endpoints.is_empty() {
            return None;
        }

        match self.strategy {
            LoadBalanceStrategy::RoundRobin => {
                let index = self.current_index.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.endpoints.get(index % self.endpoints.len())
            }
            LoadBalanceStrategy::Random => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..self.endpoints.len());
                self.endpoints.get(index)
            }
            LoadBalanceStrategy::LeastConnections => {
                // For now, use round-robin as fallback
                let index = self.current_index.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.endpoints.get(index % self.endpoints.len())
            }
        }
    }
}

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitState {
    Closed,   // Normal operation
    Open,     // Failing, reject requests
    HalfOpen, // Testing if service recovered
}

/// Circuit breaker
pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    failure_threshold: u32,
    failure_count: Arc<std::sync::atomic::AtomicU32>,
    success_threshold: u32,
    success_count: Arc<std::sync::atomic::AtomicU32>,
    timeout: Duration,
    last_failure_time: Arc<RwLock<Option<std::time::Instant>>>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, timeout: Duration) -> Self {
        Self {
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_threshold,
            failure_count: Arc::new(std::sync::atomic::AtomicU32::new(0)),
            success_threshold: 3,
            success_count: Arc::new(std::sync::atomic::AtomicU32::new(0)),
            timeout,
            last_failure_time: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn call<F, Fut, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let state = *self.state.read().await;
        
        match state {
            CircuitState::Open => {
                // Check if timeout has passed
                let last_failure = *self.last_failure_time.read().await;
                if let Some(last) = last_failure {
                    if last.elapsed() >= self.timeout {
                        // Move to half-open
                        *self.state.write().await = CircuitState::HalfOpen;
                        self.success_count.store(0, std::sync::atomic::Ordering::Relaxed);
                    } else {
                        return Err(rf_errors::RfError::Network("Circuit breaker is open".to_string()));
                    }
                } else {
                    return Err(rf_errors::RfError::Network("Circuit breaker is open".to_string()));
                }
            }
            CircuitState::HalfOpen => {
                // Allow request to test recovery
            }
            CircuitState::Closed => {
                // Normal operation
            }
        }

        match f().await {
            Ok(result) => {
                // Success
                if state == CircuitState::HalfOpen {
                    let success = self.success_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                    if success >= self.success_threshold {
                        *self.state.write().await = CircuitState::Closed;
                        self.failure_count.store(0, std::sync::atomic::Ordering::Relaxed);
                    }
                } else {
                    self.failure_count.store(0, std::sync::atomic::Ordering::Relaxed);
                }
                Ok(result)
            }
            Err(e) => {
                // Failure
                if state == CircuitState::HalfOpen {
                    *self.state.write().await = CircuitState::Open;
                    *self.last_failure_time.write().await = Some(std::time::Instant::now());
                } else {
                    let failures = self.failure_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                    if failures >= self.failure_threshold {
                        *self.state.write().await = CircuitState::Open;
                        *self.last_failure_time.write().await = Some(std::time::Instant::now());
                    }
                }
                Err(e)
            }
        }
    }
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: None,
            retry_config: RetryConfig::default(),
            load_balancer: None,
            circuit_breaker: None,
        }
    }

    /// Set base URL
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = Some(url);
        self
    }

    /// Set retry configuration
    pub fn with_retry(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Set load balancer
    pub fn with_load_balancer(mut self, balancer: LoadBalancer) -> Self {
        self.load_balancer = Some(balancer);
        self
    }

    /// Set circuit breaker
    pub fn with_circuit_breaker(mut self, breaker: Arc<CircuitBreaker>) -> Self {
        self.circuit_breaker = Some(breaker);
        self
    }

    /// Make a GET request with retry
    pub async fn get(&self, url: &str) -> Result<reqwest::Response> {
        self.request_with_retry(|client| {
            let url = self.resolve_url(url);
            client.get(&url)
        }).await
    }

    /// Make a POST request with retry
    pub async fn post(&self, url: &str, body: &str) -> Result<reqwest::Response> {
        self.request_with_retry(|client| {
            let url = self.resolve_url(url);
            client.post(&url).body(body.to_string())
        }).await
    }

    /// Resolve URL (with load balancing if configured)
    fn resolve_url(&self, url: &str) -> String {
        if let Some(ref balancer) = self.load_balancer {
            if let Some(base) = balancer.next() {
                if url.starts_with("http://") || url.starts_with("https://") {
                    return url.to_string();
                }
                return format!("{}{}", base, url);
            }
        }
        
        if let Some(ref base) = self.base_url {
            if url.starts_with("http://") || url.starts_with("https://") {
                return url.to_string();
            }
            return format!("{}{}", base, url);
        }
        
        url.to_string()
    }

    /// Make request with retry and circuit breaker
    async fn request_with_retry<F>(&self, builder: F) -> Result<reqwest::Response>
    where
        F: Fn(&Client) -> RequestBuilder,
    {
        let mut last_error = None;
        
        for attempt in 0..=self.retry_config.max_retries {
            let result = if let Some(ref breaker) = self.circuit_breaker {
                breaker.call(|| async {
                    let request = builder(&self.client);
                    request.send().await
                        .map_err(|e| rf_errors::RfError::Network(format!("Request failed: {}", e)))
                }).await
            } else {
                let request = builder(&self.client);
                request.send().await
                    .map_err(|e| rf_errors::RfError::Network(format!("Request failed: {}", e)))
            };

            match result {
                Ok(response) => {
                    let status = response.status();
                    if self.retry_config.retry_on_status.contains(&status.as_u16()) && attempt < self.retry_config.max_retries {
                        tokio::time::sleep(self.retry_config.retry_delay).await;
                        last_error = Some(rf_errors::RfError::Network(format!("HTTP {}", status)));
                        continue;
                    }
                    return Ok(response);
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.retry_config.max_retries {
                        tokio::time::sleep(self.retry_config.retry_delay).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| rf_errors::RfError::Network("Request failed after retries".to_string())))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

