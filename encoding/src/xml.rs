//! # xml
//!
//! xml 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! # XML 编码/解码模块
//!
//! 提供 XML（Extensible Markup Language）格式的序列化和反序列化功能。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{xml_encode, xml_decode, xml_pretty};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct Person {
//!     #[serde(rename = "@id")]
//!     id: String,
//!     name: String,
//!     age: u32,
//! }
//!
//! // 编码
//! let person = Person {
//!     id: "001".to_string(),
//!     name: "张三".to_string(),
//!     age: 25,
//! };
//! let xml = xml_encode(&person).unwrap();
//!
//! // 解码
//! let decoded: Person = xml_decode(&xml).unwrap();
//!
//! // 格式化
//! let pretty = xml_pretty(&xml).unwrap();
//! ```
//!
//! ## 高级功能
//!
//! - 命名空间支持
//! - XML 验证
//! - XPath 查询
//! - 与 JSON 转换
//! - 特殊字符转义

use rf_errors::Result;
use serde::{Deserialize, Serialize};
use quick_xml::se::to_string;
use quick_xml::de::from_str;

/// 将实现了 `Serialize` trait 的结构体编码为 XML 字符串
///
/// # 参数
///
/// * `value` - 要序列化的值引用，必须实现了 `Serialize` trait
///
/// # 返回值
///
/// 返回 XML 字符串，序列化失败时返回错误
///
/// # 错误
///
/// 当值包含无法序列化的类型或数据结构不合法时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xml_encode;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Data {
///     #[serde(rename = "$value")]
///     content: String,
/// }
///
/// let data = Data { content: "测试".to_string() };
/// let xml = xml_encode(&data).unwrap();
/// ```
pub fn encode<T: Serialize>(value: &T) -> Result<String> {
    to_string(value)
        .map_err(|e| rf_errors::RfError::Serialization(format!("XML encoding failed: {}", e)))
}

/// 将实现了 `Serialize` trait 的结构体编码为 XML 字符串，并指定根元素名称
///
/// 注意：quick-xml 的序列化通过 serde 属性处理根元素命名，
/// 这是一个便捷封装函数，目前使用标准的 encode 函数。
///
/// # 参数
///
/// * `value` - 要序列化的值引用
/// * `_root_name` - 根元素名称（当前未使用，应通过 serde 属性指定）
///
/// # 返回值
///
/// 返回 XML 字符串
///
/// # 注意
///
/// 根元素名称应通过 serde 属性指定，例如：
/// ```rust
/// #[derive(Serialize)]
/// #[serde(rename = "root")]
/// struct MyData {
///     field: String,
/// }
/// ```
pub fn encode_with_root<T: Serialize>(value: &T, _root_name: &str) -> Result<String> {
    // 目前使用标准编码 - 根元素名称应通过 serde 属性指定
    encode(value)
}

/// 从 XML 字符串解码为指定的结构体类型
///
/// # 类型参数
///
/// * `T` - 目标类型，必须实现了 `Deserialize` trait
///
/// # 参数
///
/// * `xml` - XML 字符串
///
/// # 返回值
///
/// 返回反序列化后的结构体实例，解析失败时返回错误
///
/// # 错误
///
/// 当 XML 格式不正确、数据类型不匹配或缺少必需字段时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xml_decode;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Data {
///     field: String,
/// }
///
/// let xml = r#"<Data><field>测试</field></Data>"#;
/// let data: Data = xml_decode(xml).unwrap();
/// ```
pub fn decode<T: for<'de> Deserialize<'de>>(xml: &str) -> Result<T> {
    from_str(xml)
        .map_err(|e| rf_errors::RfError::Serialization(format!("XML decoding failed: {}", e)))
}

/// 格式化 XML 字符串，添加缩进和换行
///
/// # 参数
///
/// * `xml` - 未格式化的 XML 字符串
///
/// # 返回值
///
/// 返回格式化后的 XML 字符串（使用 2 个空格缩进），操作失败时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xml_pretty;
///
/// let xml = "<root><item>数据</item></root>";
/// let pretty = xml_pretty(xml).unwrap();
/// // 输出:
/// // <root>
/// //   <item>数据</item>
/// // </root>
/// ```
pub fn pretty(xml: &str) -> Result<String> {
    use quick_xml::events::Event;
    use quick_xml::Reader;
    use quick_xml::Writer;
    use std::io::Cursor;

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);

    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(e) => {
                if let Err(e) = writer.write_event(e) {
                    return Err(rf_errors::RfError::Serialization(format!("XML pretty print failed: {}", e)));
                }
            }
            Err(e) => {
                return Err(rf_errors::RfError::Serialization(format!("XML pretty print failed: {}", e)));
            }
        }
    }

    let result = writer.into_inner().into_inner();
    String::from_utf8(result)
        .map_err(|e| rf_errors::RfError::Serialization(format!("XML pretty print failed: {}", e)))
}

/// 转义 XML 中的特殊字符
///
/// 将 `<`, `>`, `&`, `"`, `'` 等特殊字符转换为 XML 实体。
///
/// # 参数
///
/// * `xml` - 包含特殊字符的字符串
///
/// # 返回值
///
/// 返回转义后的字符串
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xml_escape;
///
/// let escaped = xml_escape("<data>Tom & Jerry</data>");
/// // 结果: "&lt;data&gt;Tom &amp; Jerry&lt;/data&gt;"
/// ```
pub fn escape(xml: &str) -> String {
    quick_xml::escape::escape(xml).into_owned()
}

/// 反转义 XML 中的特殊字符
///
/// 将 XML 实体转换回原始字符。
///
/// # 参数
///
/// * `xml` - 包含 XML 实体的字符串
///
/// # 返回值
///
/// 返回反转义后的字符串，转换失败时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xml_unescape;
///
/// let unescaped = xml_unescape("&lt;data&gt;").unwrap();
/// // 结果: "<data>"
/// ```
pub fn unescape(xml: &str) -> Result<String> {
    quick_xml::escape::unescape(xml)
        .map(|cow| cow.into_owned())
        .map_err(|e| rf_errors::RfError::Serialization(format!("XML unescape failed: {}", e)))
}

/// XML 命名空间信息
///
/// 用于表示 XML 命名空间的前缀和 URI。
///
/// # 字段
///
/// * `prefix` - 命名空间前缀（如 "xs", "xsi"）
/// * `uri` - 命名空间 URI（如 "http://www.w3.org/2001/XMLSchema"）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xml_namespace::Namespace;
///
/// let ns = Namespace {
///     prefix: "xs".to_string(),
///     uri: "http://www.w3.org/2001/XMLSchema".to_string(),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Namespace {
    /// 命名空间前缀
    pub prefix: String,
    /// 命名空间 URI
    pub uri: String,
}

/// 使用命名空间支持将值编码为 XML
///
/// # 参数
///
/// * `value` - 要序列化的值引用
/// * `namesspaces` - 命名空间列表
///
/// # 返回值
///
/// 返回包含命名空间声明的 XML 字符串
///
/// # 注意
///
/// 这是一个简化实现，完整的命名空间支持需要使用 quick-xml 的命名空间功能。
///
/// # 示例
///
/// ```rust
/// use rf_encoding::{xml_encode_with_namespace, xml_namespace::Namespace};
///
/// let ns = Namespace {
///     prefix: "xs".to_string(),
///     uri: "http://www.w3.org/2001/XMLSchema".to_string(),
/// };
/// let xml = xml_encode_with_namespace(&data, &[ns]).unwrap();
/// ```
pub fn encode_with_namespace<T: Serialize>(value: &T, namespaces: &[Namespace]) -> Result<String> {
    // 这是简化实现
    // 完整实现会使用 quick-xml 的命名空间支持
    let mut xml = encode(value)?;

    // 将命名空间声明添加到根元素
    if let Some(root_start) = xml.find('<') {
        let mut ns_decls = String::new();
        for ns in namespaces {
            ns_decls.push_str(&format!(r#" xmlns:{}="{}""#, ns.prefix, ns.uri));
        }
        xml.insert_str(root_start + 1, &ns_decls);
    }

    Ok(xml)
}

/// 验证 XML 结构（基础验证）
///
/// 检查 XML 标签是否正确匹配和闭合。
///
/// # 参数
///
/// * `xml` - 要验证的 XML 字符串
///
/// # 返回值
///
/// 如果 XML 结构有效则返回 Ok(())，否则返回错误
///
/// # 错误
///
/// - 当开始和结束标签不匹配时
/// - 当标签未正确闭合时
/// - 当 XML 格式无效时
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xml_validate;
///
/// // 有效的 XML
/// xml_validate("<root><item>数据</item></root>").unwrap();
///
/// // 无效的 XML（标签不匹配）
/// assert!(xml_validate("<root><item></root></item>").is_err());
/// ```
pub fn validate(xml: &str) -> Result<()> {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut depth = 0;
    loop {
        match reader.read_event() {
            Ok(Event::Start(_)) => depth += 1,
            Ok(Event::End(_)) => {
                depth -= 1;
                if depth < 0 {
                    return Err(rf_errors::RfError::Serialization("Mismatched XML tags".to_string()));
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => {
                return Err(rf_errors::RfError::Serialization(format!("XML validation error: {}", e)));
            }
        }
    }

    if depth != 0 {
        return Err(rf_errors::RfError::Serialization("Unclosed XML tags".to_string()));
    }

    Ok(())
}

/// 将 XML 转换为 JSON（简化实现）
///
/// # 参数
///
/// * `xml` - XML 字符串
///
/// # 返回值
///
/// 返回 JSON Value，转换失败时返回错误
///
/// # 注意
///
/// 这是一个简化实现，完整的实现会正确解析 XML 并转换为 JSON。
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xml_to_json;
///
/// let xml = "<root><name>张三</name><age>25</age></root>";
/// let json = xml_to_json(xml).unwrap();
/// ```
pub fn to_json(xml: &str) -> Result<serde_json::Value> {
    // 这是简化实现
    // 完整实现会正确解析 XML 并转换为 JSON
    use quick_xml::Reader;
    use quick_xml::events::Event;
    use std::collections::HashMap;

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut stack: Vec<serde_json::Value> = Vec::new();
    let mut current: Option<HashMap<String, serde_json::Value>> = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let _name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if let Some(map) = current.take() {
                    stack.push(serde_json::Value::Object(map.into_iter().collect()));
                }
                current = Some(HashMap::new());
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if let Some(ref mut map) = current {
                    map.insert("_text".to_string(), serde_json::Value::String(text));
                }
            }
            Ok(Event::End(_)) => {
                if let Some(map) = current.take() {
                    if let Some(mut parent) = stack.pop() {
                        if let Some(parent_obj) = parent.as_object_mut() {
                            // 简化：直接添加到父对象
                            parent_obj.extend(map);
                        }
                        stack.push(parent);
                    } else {
                        stack.push(serde_json::Value::Object(map.into_iter().collect()));
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => {
                return Err(rf_errors::RfError::Serialization(format!("XML to JSON conversion error: {}", e)));
            }
        }
    }

    stack.pop()
            .ok_or_else(|| rf_errors::RfError::Serialization("Empty XML document".to_string()))
}

/// XPath 查询支持（简化实现 - 基本元素选择）
///
/// # 参数
///
/// * `xml` - XML 字符串
/// * `path` - XPath 路径（简化版本，仅支持简单路径）
///
/// # 返回值
///
/// 返回匹配元素的文本内容列表
///
/// # 注意
///
/// 这是一个简化的 XPath 实现，仅支持基本的路径选择。
/// 完整实现会使用专业的 XPath 库，如 sxd-xpath。
///
/// # 示例
///
/// ```rust
/// use rf_encoding::xml_xpath_query;
///
/// let xml = r#"
/// <root>
///     <user><name>张三</name></user>
///     <user><name>李四</name></user>
/// </root>
/// "#;
/// let results = xml_xpath_query(xml, "root/user/name").unwrap();
/// // 结果: ["张三", "李四"]
/// ```
pub fn xpath_query(xml: &str, path: &str) -> Result<Vec<String>> {
    // 这是简化的 XPath 实现
    // 完整实现会使用专业的 XPath 库，如 sxd-xpath
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let path_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let mut results = Vec::new();
    let mut current_path = Vec::new();
    let mut current_text = String::new();
    let mut in_target = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                current_path.push(name);

                // 检查是否在目标路径中
                if current_path.len() == path_parts.len() {
                    let matches = current_path.iter().zip(path_parts.iter())
                        .all(|(a, b)| a == b);
                    if matches {
                        in_target = true;
                    }
                }
            }
            Ok(Event::Text(e)) => {
                if in_target {
                    current_text.push_str(&e.unescape().unwrap_or_default());
                }
            }
            Ok(Event::End(_)) => {
                if in_target && !current_text.trim().is_empty() {
                    results.push(current_text.trim().to_string());
                    current_text.clear();
                }
                in_target = false;
                current_path.pop();
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => {
                return Err(rf_errors::RfError::Serialization(format!("XPath query error: {}", e)));
            }
        }
    }

    Ok(results)
}
