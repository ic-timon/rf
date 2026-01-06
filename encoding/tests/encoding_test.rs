//! Encoding module tests

use rf_encoding::{json_encode, json_decode, base64_encode, base64_decode};
use rf_encoding::url::{url_encode, url_decode};

#[test]
fn test_json_encode_decode() {
    let data = serde_json::json!({"name": "test", "value": 42});
    let encoded = json_encode(&data).unwrap();
    assert!(!encoded.is_empty());
    
    let decoded: serde_json::Value = json_decode(&encoded).unwrap();
    assert_eq!(decoded["name"], "test");
    assert_eq!(decoded["value"], 42);
}

#[test]
fn test_base64_encode_decode() {
    let data = b"Hello, world!";
    let encoded = base64_encode(data);
    assert!(!encoded.is_empty());
    
    let decoded = base64_decode(&encoded).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn test_url_encode_decode() {
    let data = "hello world";
    let encoded = url_encode(data);
    assert!(encoded.contains("%20")); // Space should be encoded
    
    let decoded = url_decode(&encoded).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn test_url_encode_special_chars() {
    let data = "hello&world=test";
    let encoded = url_encode(data);
    let decoded = url_decode(&encoded).unwrap();
    assert_eq!(decoded, data);
}

