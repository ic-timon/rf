//! Crypto module tests

use rf_crypto::{aes, md5, sha1, sha256, crc32};

#[test]
fn test_md5_hash() {
    let data = b"Hello, world!";
    let hash = md5::hash(data);
    assert_eq!(hash.len(), 32); // MD5 produces 32 hex characters
    assert!(!hash.is_empty());
}

#[test]
fn test_sha1_hash() {
    let data = b"Hello, world!";
    let hash = sha1::hash(data);
    assert_eq!(hash.len(), 40); // SHA-1 produces 40 hex characters
    assert!(!hash.is_empty());
}

#[test]
fn test_sha256_hash() {
    let data = b"Hello, world!";
    let hash = sha256::hash(data);
    assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex characters
    assert!(!hash.is_empty());
}

#[test]
fn test_crc32_checksum() {
    let data = b"Hello, world!";
    let checksum = crc32::checksum(data);
    assert!(checksum > 0);
}

#[test]
fn test_crc32_consistency() {
    let data = b"Hello, world!";
    let checksum1 = crc32::checksum(data);
    let checksum2 = crc32::checksum(data);
    assert_eq!(checksum1, checksum2); // Same input should produce same checksum
}

#[test]
fn test_aes_encrypt_decrypt() {
    let key = [0u8; 32]; // 256-bit key
    let nonce = [0u8; 12]; // 96-bit nonce
    let plaintext = b"Hello, world!";
    
    let encrypted = aes::encrypt(&key, &nonce, plaintext);
    assert!(encrypted.is_ok());
    
    let encrypted_data = encrypted.unwrap();
    let decrypted = aes::decrypt(&key, &nonce, &encrypted_data);
    assert!(decrypted.is_ok());
    assert_eq!(decrypted.unwrap(), plaintext);
}

