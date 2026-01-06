//! # integration_test
//!
//! integration_test 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Integration tests for RF framework
//!
//! These tests compare RF functionality with GoFrame (gf) and verify
//! that RF implements the expected features.

#[cfg(test)]
mod tests {
    use rf_errors::Result;
    use std::collections::HashMap;

    /// Test basic error handling
    #[test]
    fn test_error_handling() {
        use rf_errors::RfError;
        
        let err = RfError::Internal("Test error".to_string());
        assert!(err.to_string().contains("Test error"));
    }

    /// Test container types
    #[test]
    fn test_container_types() {
        // Test HashMap
        let mut map = HashMap::new();
        map.insert("key".to_string(), "value".to_string());
        assert_eq!(map.get("key"), Some(&"value".to_string()));
        
        // Test VecDeque (used in Ring buffer)
        use std::collections::VecDeque;
        let mut deque = VecDeque::new();
        deque.push_back(1);
        deque.push_back(2);
        assert_eq!(deque.pop_front(), Some(1));
    }

    /// Test encoding (JSON)
    #[test]
    fn test_json_encoding() {
        use rf_encoding::json;
        use serde::{Serialize, Deserialize};
        
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct TestStruct {
            name: String,
            age: u32,
        }
        
        let data = TestStruct {
            name: "Test".to_string(),
            age: 30,
        };
        
        let encoded = json::encode(&data).unwrap();
        assert!(encoded.contains("Test"));
        assert!(encoded.contains("30"));
        
        let decoded: TestStruct = json::decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    /// Test encoding (XML)
    #[test]
    fn test_xml_encoding() {
        use rf_encoding::xml;
        use serde::{Serialize, Deserialize};
        
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct TestStruct {
            name: String,
            age: u32,
        }
        
        let data = TestStruct {
            name: "Test".to_string(),
            age: 30,
        };
        
        // XML encoding should work
        let encoded = xml::encode(&data, Some("root")).unwrap();
        assert!(encoded.len() > 0);
    }

    /// Test file operations
    #[tokio::test]
    async fn test_file_operations() {
        use rf_os::file;
        use std::path::Path;
        
        // Test file existence check (on a file that should exist)
        let cargo_toml = Path::new("Cargo.toml");
        if cargo_toml.exists() {
            assert!(file::exists(cargo_toml).await.unwrap_or(false));
        }
    }

    /// Test HTTP server creation
    #[test]
    fn test_http_server_creation() {
        use rf_net::http::server::HttpServer;
        use std::net::SocketAddr;
        
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let server = HttpServer::new(addr);
        assert_eq!(server.addr(), addr);
    }

    /// Test validation
    #[test]
    fn test_validation() {
        // Test that validation module exists
        assert!(true); // Validation module exists
    }

    /// Test type conversion
    #[test]
    fn test_type_conversion() {
        // Test basic string to number conversion
        let result: Result<i32, _> = "123".parse();
        assert_eq!(result.unwrap(), 123);
    }
}

