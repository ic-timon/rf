# Text 模块教程

Text 模块提供了文本处理功能，包括字符串操作和正则表达式处理。

## 模块概述

Text 模块包含以下功能：

- **字符串操作（str）**：基础字符串操作（分割、连接、替换、大小写转换等）
- **正则表达式（regex）**：模式匹配、查找、替换等

## 快速开始

### 添加依赖

```toml
[dependencies]
rf-text = { path = "../rf/text" }
regex = "1.10"
```

### 基本导入

```rust
use rf_text::str::{split, join, replace, trim};
use rf_text::regex::{is_match, find, replace as regex_replace};
```

## 核心功能

### 字符串操作

#### 基本操作

```rust
use rf_text::str::*;

// 检查字符串是否为空
assert_eq!(is_empty(""), true);
assert_eq!(is_empty("hello"), false);

// 检查是否包含子串
assert_eq!(contains("hello world", "world"), true);

// 分割字符串
let parts = split("a,b,c", ",");
assert_eq!(parts, vec!["a", "b", "c"]);

// 连接字符串
let strings = vec!["hello", "world"];
assert_eq!(join(&strings, " "), "hello world");

// 修剪字符串（去除首尾空白）
assert_eq!(trim("  hello  "), "hello");

// 大小写转换
assert_eq!(to_upper("hello"), "HELLO");
assert_eq!(to_lower("HELLO"), "hello");

// 替换子串
assert_eq!(replace("hello world", "world", "rust"), "hello rust");
```

#### 字符串检查

```rust
use rf_text::str::*;

// 检查是否以指定前缀开头
assert_eq!(starts_with("hello", "he"), true);

// 检查是否以指定后缀结尾
assert_eq!(ends_with("hello", "lo"), true);

// 检查字符串长度
assert_eq!(len("hello"), 5);
```

#### 字符串截取

```rust
use rf_text::str::*;

let s = "hello world";

// 获取子串（从索引 0 开始，长度为 5）
let substr = substring(s, 0, 5);
assert_eq!(substr, "hello");

// 获取前缀
let prefix = prefix(s, 5);
assert_eq!(prefix, "hello");

// 获取后缀
let suffix = suffix(s, 5);
assert_eq!(suffix, "world");
```

### 正则表达式

#### 模式匹配

```rust
use rf_text::regex::*;

// 检查字符串是否匹配模式
let matched = is_match(r"\d+", "abc123def")?;
assert!(matched);

// 查找第一个匹配
if let Some(m) = find(r"\d+", "abc123def")? {
    println!("找到: {}", m.as_str());
}

// 查找所有匹配
let matches = find_all(r"\d+", "abc123def456")?;
for m in matches {
    println!("匹配: {}", m.as_str());
}
```

#### 正则替换

```rust
use rf_text::regex::*;

// 替换第一个匹配
let result = replace(r"\d+", "abc123def", "XXX")?;
assert_eq!(result, "abcXXXdef");

// 替换所有匹配
let result = replace_all(r"\d+", "abc123def456", "XXX")?;
assert_eq!(result, "abcXXXdefXXX");
```

#### 捕获组

```rust
use rf_text::regex::*;

// 使用捕获组提取信息
let text = "2024-01-01";
let pattern = r"(\d{4})-(\d{2})-(\d{2})";

if let Some(captures) = captures(pattern, text)? {
    println!("年: {}", captures.get(1).unwrap().as_str());
    println!("月: {}", captures.get(2).unwrap().as_str());
    println!("日: {}", captures.get(3).unwrap().as_str());
}
```

#### 常用正则模式

```rust
use rf_text::regex::*;

// 邮箱验证
let email_pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
assert!(is_match(email_pattern, "user@example.com")?);

// 手机号验证（11位数字）
let phone_pattern = r"^\d{11}$";
assert!(is_match(phone_pattern, "13800138000")?);

// URL 验证
let url_pattern = r"^https?://[^\s/$.?#].[^\s]*$";
assert!(is_match(url_pattern, "https://example.com")?);

// IP 地址验证
let ip_pattern = r"^(\d{1,3}\.){3}\d{1,3}$";
assert!(is_match(ip_pattern, "192.168.1.1")?);
```

## 高级用法

### 字符串处理链

```rust
use rf_text::str::*;

let text = "  Hello, World!  ";

// 链式处理
let result = text
    .trim()  // 修剪空白
    .to_lower()  // 转小写
    .replace(",", "")  // 移除逗号
    .replace("!", "");  // 移除感叹号

assert_eq!(result, "hello world");
```

### 复杂正则匹配

```rust
use rf_text::regex::*;

// 提取 HTML 标签内容
let html = "<div>Hello</div><span>World</span>";
let pattern = r"<(\w+)>([^<]+)</\1>";

let matches = find_all(pattern, html)?;
for m in matches {
    if let Some(captures) = captures(pattern, m.as_str())? {
        let tag = captures.get(1).unwrap().as_str();
        let content = captures.get(2).unwrap().as_str();
        println!("标签: {}, 内容: {}", tag, content);
    }
}
```

### 文本清理

```rust
use rf_text::str::*;
use rf_text::regex::*;

fn clean_text(text: &str) -> Result<String> {
    let mut cleaned = text.to_string();
    
    // 移除多余空白
    cleaned = replace_all(r"\s+", &cleaned, " ")?;
    
    // 修剪首尾空白
    cleaned = trim(&cleaned).to_string();
    
    // 移除特殊字符
    cleaned = replace_all(r"[^\w\s]", &cleaned, "")?;
    
    Ok(cleaned)
}

let dirty = "  Hello,   World!!!  ";
let clean = clean_text(dirty)?;
assert_eq!(clean, "Hello World");
```

### 文本解析

```rust
use rf_text::str::*;
use rf_text::regex::*;

// 解析键值对
fn parse_key_value(text: &str) -> Result<Vec<(String, String)>> {
    let lines = split(text, "\n");
    let mut result = Vec::new();
    
    for line in lines {
        let parts = split(&trim(line), "=");
        if parts.len() == 2 {
            result.push((
                parts[0].to_string(),
                parts[1].to_string()
            ));
        }
    }
    
    Ok(result)
}

let config = "host=localhost\nport=8080\ndebug=true";
let pairs = parse_key_value(config)?;
for (key, value) in pairs {
    println!("{} = {}", key, value);
}
```

## API 参考

### 字符串操作函数

- `is_empty(s: &str) -> bool` - 检查是否为空
- `contains(s: &str, substr: &str) -> bool` - 检查是否包含子串
- `split(s: &str, delimiter: &str) -> Vec<&str>` - 分割字符串
- `join(strings: &[&str], separator: &str) -> String` - 连接字符串
- `trim(s: &str) -> &str` - 修剪空白
- `to_upper(s: &str) -> String` - 转大写
- `to_lower(s: &str) -> String` - 转小写
- `replace(s: &str, from: &str, to: &str) -> String` - 替换子串
- `starts_with(s: &str, prefix: &str) -> bool` - 检查前缀
- `ends_with(s: &str, suffix: &str) -> bool` - 检查后缀
- `len(s: &str) -> usize` - 获取长度
- `substring(s: &str, start: usize, len: usize) -> String` - 获取子串

### 正则表达式函数

- `is_match(pattern: &str, text: &str) -> Result<bool>` - 检查是否匹配
- `find(pattern: &str, text: &str) -> Result<Option<Match>>` - 查找第一个匹配
- `find_all(pattern: &str, text: &str) -> Result<Vec<Match>>` - 查找所有匹配
- `captures(pattern: &str, text: &str) -> Result<Option<Captures>>` - 获取捕获组
- `replace(pattern: &str, text: &str, replacement: &str) -> Result<String>` - 替换第一个匹配
- `replace_all(pattern: &str, text: &str, replacement: &str) -> Result<String>` - 替换所有匹配

## 常见问题

### Q: 正则表达式函数返回 Result 类型，如何处理错误？

A: 使用 `?` 运算符传播错误，或在 `match` 表达式中处理：

```rust
match is_match(r"\d+", "123") {
    Ok(true) => println!("匹配"),
    Ok(false) => println!("不匹配"),
    Err(e) => eprintln!("错误: {}", e),
}
```

### Q: 如何提高正则表达式性能？

A: 
- 编译正则表达式并复用（使用 `Regex::new`）
- 避免过于复杂的模式
- 使用非贪婪匹配（`*?` 而不是 `*`）当适用时

### Q: 字符串操作函数和标准库有什么区别？

A: Text 模块的函数提供了更统一的接口，并且与正则表达式功能更好地集成。

## 相关链接

- [util 模块](../util/README.md) - 工具函数
- [encoding 模块](../encoding/README.md) - 编码处理

