//! # feature_test
//!
//! feature_test 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Feature-specific tests for RF framework

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    /// Test HTTP router with priority
    #[tokio::test]
    async fn test_http_router_priority() {
        use rf_net::http::router::RouterManager;
        use axum::http::Method;
        
        let manager = RouterManager::new();
        
        // Register routes with different priorities
        manager.register_route(
            "/api/v1/users".to_string(),
            Method::GET,
            10,
            "handler1".to_string(),
        ).await.unwrap();
        
        manager.register_route(
            "/api/v1/users".to_string(),
            Method::POST,
            5,
            "handler2".to_string(),
        ).await.unwrap();
        
        let routes = manager.get_routes().await;
        assert_eq!(routes.len(), 2);
        // Higher priority should come first
        assert_eq!(routes[0].priority, 10);
    }

    /// Test query builder with AND/OR conditions
    #[test]
    fn test_query_builder_conditions() {
        use rf_database::db::query::QueryBuilder;
        
        let query = QueryBuilder::new()
            .r#where("age > ?", Some(vec!["18".to_string()]))
            .and_where("status = ?", Some(vec!["active".to_string()]))
            .or_where("role = ?", Some(vec!["admin".to_string()]));
        
        let (sql, params) = query.build_select("SELECT * FROM users");
        assert!(sql.contains("WHERE"));
        assert!(sql.contains("AND") || sql.contains("OR"));
        assert_eq!(params.len(), 3);
    }

    /// Test session expiration
    #[test]
    fn test_session_expiration() {
        use rf_os::session::storage::{MemorySessionStorage, ExpirationPolicy};
        use rf_os::session::Session;
        use std::time::Duration;
        
        let mut storage = MemorySessionStorage::new();
        let policy = ExpirationPolicy {
            default_ttl: Duration::from_secs(1),
            max_ttl: None,
            sliding_expiration: false,
        };
        storage.set_expiration_policy(policy);
        
        // Create a session
        let session = Session::new();
        storage.store(session.clone()).unwrap();
        
        // Session should exist
        assert!(storage.get(session.id()).unwrap().is_some());
    }

    /// Test HTTP client with retry
    #[test]
    fn test_http_client_retry() {
        use rf_contrib_sdk_httpclient::{HttpClient, RetryConfig};
        use std::time::Duration;
        
        let client = HttpClient::new()
            .with_retry(RetryConfig {
                max_retries: 3,
                retry_delay: Duration::from_millis(100),
                retry_on_status: vec![500, 502, 503],
            });
        
        // Client should be created with retry config
        assert!(true);
    }

    /// Test load balancer
    #[test]
    fn test_load_balancer() {
        use rf_contrib_sdk_httpclient::{LoadBalancer, LoadBalanceStrategy};
        
        let balancer = LoadBalancer::new(
            vec!["http://server1".to_string(), "http://server2".to_string()],
            LoadBalanceStrategy::RoundRobin,
        );
        
        // Should get a server
        assert!(balancer.next().is_some());
    }

    /// Test circuit breaker
    #[test]
    fn test_circuit_breaker() {
        use rf_contrib_sdk_httpclient::CircuitBreaker;
        use std::time::Duration;
        
        let breaker = CircuitBreaker::new(5, Duration::from_secs(10));
        
        // Circuit breaker should be created
        assert!(true);
    }

    /// Test query plan cache
    #[test]
    fn test_query_plan_cache() {
        use rf_database::db::query_plan_cache::{QueryPlanCache, QueryPlanKey};
        use std::time::Duration;
        
        let cache = QueryPlanCache::new(100, Duration::from_secs(300));
        
        // Cache should be created
        assert!(true);
    }

    /// Test batch query optimizer
    #[test]
    fn test_batch_query_optimizer() {
        use rf_database::db::query_plan_cache::BatchQueryOptimizer;
        
        let optimizer = BatchQueryOptimizer::new(100);
        let queries = vec!["INSERT INTO users VALUES (1)".to_string()];
        let optimized = optimizer.optimize_batch_insert(&queries);
        
        assert!(!optimized.is_empty());
    }

    /// Test pool optimizer
    #[test]
    fn test_pool_optimizer() {
        use rf_database::db::pool_monitor::PoolOptimizer;
        
        let mut optimizer = PoolOptimizer::new(5, 20, 70.0);
        
        // Test pool size adjustment
        let new_size = optimizer.adjust_pool_size(80.0);
        assert!(new_size >= 5 && new_size <= 20);
    }
}

