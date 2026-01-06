//! Net module tests

use rf_net::{TcpClient, TcpServer, UdpSocketWrapper, RoundRobinSelector, Selector};

#[tokio::test]
async fn test_tcp_server_bind() {
    let server = TcpServer::bind("127.0.0.1:0").await;
    assert!(server.is_ok());
}

#[tokio::test]
async fn test_udp_socket_bind() {
    let socket = UdpSocketWrapper::bind("127.0.0.1:0").await;
    assert!(socket.is_ok());
}

#[test]
fn test_round_robin_selector() {
    let mut selector = RoundRobinSelector::new();
    let items = vec!["a", "b", "c"];
    
    assert_eq!(selector.select(&items), Some(&"a"));
    assert_eq!(selector.select(&items), Some(&"b"));
    assert_eq!(selector.select(&items), Some(&"c"));
    assert_eq!(selector.select(&items), Some(&"a")); // Wraps around
}

#[test]
fn test_round_robin_selector_empty() {
    let mut selector = RoundRobinSelector::new();
    let items: Vec<i32> = vec![];
    
    assert_eq!(selector.select(&items), None);
}

