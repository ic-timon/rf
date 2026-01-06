# Encoding 模块教程

Encoding 模块提供各种编码和解码功能，支持多种数据格式和编码方式。

## 模块概述

Encoding 模块支持：

- **数据序列化**：JSON、YAML、TOML、XML
- **配置文件**：INI、Properties
- **编码格式**：Base64、Binary、Charset
- **压缩格式**：Gzip、Zlib
- **哈希算法**：XXHash
- **Web 格式**：HTML、URL

## 快速开始

### 添加依赖

```toml
[dependencies]
rf-encoding = { path = "../rf/encoding" }
serde_json = "1.0"
```

### 基本导入

```rust
use rf_encoding::{json_encode, json_decode, base64_encode, base64_decode};
```

## 核心功能

### JSON 编码/解码

```rust
use rf_encoding::{json_encode, json_decode, json_encode_pretty};
use serde_json::json;

// 编码
let data = json!({"name": "RF", "version": "0.1.0"});
let json_str = json_encode(&data)?;
println!("{}", json_str);

// 美化编码
let pretty = json_encode_pretty(&data)?;

// 解码
let decoded: serde_json::Value = json_decode(&json_str)?;
```

### YAML 编码/解码

```rust
use rf_encoding::{yaml_encode, yaml_decode};

let data = json!({"key": "value"});
let yaml_str = yaml_encode(&data)?;

let decoded: serde_json::Value = yaml_decode(&yaml_str)?;
```

### TOML 编码/解码

```rust
use rf_encoding::{toml_encode, toml_decode};

let data = json!({"title": "RF Framework"});
let toml_str = toml_encode(&data)?;

let decoded: serde_json::Value = toml_decode(&toml_str)?;
```

### XML 编码/解码

```rust
use rf_encoding::{xml_encode, xml_decode};

let data = json!({"user": {"name": "Alice"}});
let xml_str = xml_encode(&data)?;

let decoded: serde_json::Value = xml_decode(&xml_str)?;
```

### Base64 编码/解码

```rust
use rf_encoding::{base64_encode, base64_decode};

// 编码
let data = b"Hello, RF!";
let encoded = base64_encode(data);
println!("{}", encoded);

// 解码
let decoded = base64_decode(&encoded)?;
assert_eq!(decoded, data);
```

### URL 编码/解码

```rust
use rf_encoding::url;

// 解析 URL
let parsed = url::parse("https://example.com/path?key=value")?;
println!("协议: {}", url::scheme(&parsed));
println!("主机: {}", url::host(&parsed));
println!("路径: {}", url::path(&parsed));
```

### HTML 解析

```rust
use rf_encoding::html;

let html_content = "<div><h1>Title</h1><p>Content</p></div>";

// 解析 HTML
let doc = html::parse(html_content)?;

// 提取文本
let text = html::extract_text(&doc);
println!("{}", text);

// 选择元素
let elements = html::select(&doc, "h1")?;
```

### 压缩

```rust
use rf_encoding::compress;

let data = b"Hello, RF Framework!";

// Gzip 压缩
let compressed = compress::gzip_compress(data)?;

// Gzip 解压
let decompressed = compress::gzip_decompress(&compressed)?;
assert_eq!(decompressed, data);
```

### 哈希

```rust
use rf_encoding::hash;

let data = b"Hello, RF!";

// XXHash
let hash = hash::xxhash(data);
println!("{:x}", hash);
```

## API 参考

### JSON

- `json_encode(value: &Value) -> Result<String>` - 编码
- `json_encode_pretty(value: &Value) -> Result<String>` - 美化编码
- `json_decode(s: &str) -> Result<Value>` - 解码

### Base64

- `base64_encode(data: &[u8]) -> String` - 编码
- `base64_decode(s: &str) -> Result<Vec<u8>>` - 解码

## 相关链接

- [crypto 模块](../crypto/README.md) - 加密功能
- [text 模块](../text/README.md) - 文本处理

