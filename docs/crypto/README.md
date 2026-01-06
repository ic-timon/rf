# Crypto 模块教程

Crypto 模块提供加密和哈希功能，包括对称加密、非对称加密和多种哈希算法。

## 模块概述

Crypto 模块包含：

- **AES**：AES-256-GCM 对称加密
- **DES**：DES 加密
- **RSA**：RSA 非对称加密
- **MD5**：MD5 哈希
- **SHA1**：SHA-1 哈希
- **SHA256**：SHA-256 哈希
- **CRC32**：CRC32 校验和

## 快速开始

### 添加依赖

```toml
[dependencies]
rf-crypto = { path = "../rf/crypto" }
```

### 基本导入

```rust
use rf_crypto::{aes, md5, sha256, crc32};
```

## 核心功能

### AES 加密

```rust
use rf_crypto::aes;

// 准备密钥和 nonce
let key = [0u8; 32];  // 256位密钥
let nonce = [0u8; 12]; // 96位nonce
let plaintext = b"Hello, RF!";

// 加密
let encrypted = aes::encrypt(&key, &nonce, plaintext)?;

// 解密
let decrypted = aes::decrypt(&key, &nonce, &encrypted)?;
assert_eq!(decrypted, plaintext);
```

### RSA 加密

```rust
use rf_crypto::rsa;

// 生成密钥对
let key_pair = rsa::RsaKeyPair::new(2048)?;

// 加密
let data = b"Secret message";
let encrypted = rsa::encrypt(key_pair.public_key(), data)?;

// 解密
let decrypted = rsa::decrypt(key_pair.private_key(), &encrypted)?;
assert_eq!(decrypted, data);
```

### MD5 哈希

```rust
use rf_crypto::md5;

let data = b"Hello, RF!";
let hash = md5::hash(data);
println!("MD5: {:x}", hash);
```

### SHA256 哈希

```rust
use rf_crypto::sha256;

let data = b"Hello, RF!";
let hash = sha256::hash(data);
println!("SHA256: {:x}", hash);
```

### CRC32 校验和

```rust
use rf_crypto::crc32;

let data = b"Hello, RF!";
let checksum = crc32::checksum(data);
println!("CRC32: {:x}", checksum);
```

## API 参考

### AES

- `encrypt(key: &[u8], nonce: &[u8], data: &[u8]) -> Result<Vec<u8>>` - 加密
- `decrypt(key: &[u8], nonce: &[u8], data: &[u8]) -> Result<Vec<u8>>` - 解密

### RSA

- `RsaKeyPair::new(bits: usize) -> Result<Self>` - 生成密钥对
- `encrypt(pub_key: &PublicKey, data: &[u8]) -> Result<Vec<u8>>` - 加密
- `decrypt(priv_key: &PrivateKey, data: &[u8]) -> Result<Vec<u8>>` - 解密

### 哈希

- `md5::hash(data: &[u8]) -> [u8; 16]` - MD5 哈希
- `sha256::hash(data: &[u8]) -> [u8; 32]` - SHA256 哈希
- `crc32::checksum(data: &[u8]) -> u32` - CRC32 校验和

## 相关链接

- [encoding 模块](../encoding/README.md) - 编码功能

