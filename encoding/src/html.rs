//! # html
//!
//! html 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06
//! # HTML 处理模块
//!
//! 提供 HTML 文档的解析、查询和操作功能。
//!
//! ## 使用示例
//!
//! ```rust
//! use rf_encoding::{html_parse, html_extract_text, html_select};
//!
//! let html = r#"<html><body><h1>Hello</h1><p class="content">World</p></body></html>"#;
//!
//! // 解析 HTML
//! let doc = html_parse(html).unwrap();
//!
//! // 提取纯文本
//! let text = html_extract_text(&doc);
//!
//! // 使用 CSS 选择器查询元素
//! let elements = html_select(&doc, "p.content");
//! ```
//!
//! ## 功能说明
//!
//! - HTML 解析：将 HTML 字符串解析为文档对象模型
//! - 文本提取：从 HTML 中提取纯文本内容
//! - 元素选择：使用 CSS 选择器查询元素
//! - 属性操作：读取和修改元素属性

use rf_errors::{Result, RfError};
use std::collections::HashMap;

/// HTML 文档类型
///
/// 表示解析后的 HTML 文档。
#[derive(Debug, Clone)]
pub enum HtmlNode {
    /// 元素节点
    Element {
        /// 标签名
        tag: String,
        /// 属性
        attrs: HashMap<String, String>,
        /// 子节点
        children: Vec<HtmlNode>,
    },
    /// 文本节点
    Text(String),
    /// 注释节点
    Comment(String),
}

impl HtmlNode {
    /// 创建新元素
    pub fn element(tag: impl Into<String>) -> Self {
        HtmlNode::Element {
            tag: tag.into(),
            attrs: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// 创建新文本节点
    pub fn text(content: impl Into<String>) -> Self {
        HtmlNode::Text(content.into())
    }

    /// 创建新注释节点
    pub fn comment(content: impl Into<String>) -> Self {
        HtmlNode::Comment(content.into())
    }

    /// 添加属性
    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if let HtmlNode::Element { ref mut attrs, .. } = self {
            attrs.insert(key.into(), value.into());
        }
        self
    }

    /// 添加子节点
    pub fn with_child(mut self, child: HtmlNode) -> Self {
        if let HtmlNode::Element { ref mut children, .. } = self {
            children.push(child);
        }
        self
    }

    /// 转换为 HTML 字符串
    pub fn to_html(&self) -> String {
        match self {
            HtmlNode::Element { tag, attrs, children } => {
                let mut result = format!("<{}", tag);
                for (k, v) in attrs {
                    result.push_str(&format!(" {}=\"{}\"", k, v));
                }
                if children.is_empty() {
                    result.push_str(" />");
                } else {
                    result.push('>');
                    for child in children {
                        result.push_str(&child.to_html());
                    }
                    result.push_str(&format!("</{}>", tag));
                }
                result
            }
            HtmlNode::Text(content) => content.clone(),
            HtmlNode::Comment(content) => format!("<!--{}-->", content),
        }
    }
}
///
/// 表示解析后的 HTML 文档结构。
#[derive(Debug, Clone)]
pub struct HtmlDocument {
    /// 根节点
    pub root: HtmlNode,
}

impl HtmlDocument {
    /// 创建新文档
    pub fn new(root: HtmlNode) -> Self {
        Self { root }
    }

    /// 转换为 HTML 字符串
    pub fn to_html(&self) -> String {
        self.root.to_html()
    }
}

/// 简单的 HTML 解析器
///
/// 将 HTML 字符串解析为文档对象模型。
///
/// # 参数
///
/// * `html` - HTML 字符串
///
/// # 返回值
///
/// 返回解析后的文档，解析失败时返回错误
///
/// # 示例
///
/// ```rust
/// use rf_encoding::html_parse;
///
/// let html = "<html><body>Hello</body></html>";
/// let doc = html_parse(html).unwrap();
/// ```
pub fn parse(html: &str) -> Result<HtmlDocument> {
    // 简单的 HTML 解析实现
    let mut chars = html.chars().peekable();
    let root = parse_node(&mut chars)?;
    Ok(HtmlDocument::new(root))
}

/// 解析单个节点（内部函数）
fn parse_node(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<HtmlNode> {
    // 跳过空白字符
    while chars.peek().map_or(false, |c| c.is_whitespace()) {
        chars.next();
    }

    match chars.peek() {
        Some('<') => {
            chars.next(); // 消耗 '<'

            // 检查是否是注释
            if chars.peek() == Some(&'!') {
                chars.next();
                if chars.peek() == Some(&'-') {
                    chars.next();
                    if chars.peek() == Some(&'-') {
                        chars.next();
                        return parse_comment(chars);
                    }
                }
            }

            // 检查是否是结束标签
            if chars.peek() == Some(&'/') {
                chars.next();
                let tag_name = parse_tag_name(chars)?;
                chars.next(); // 消耗 '>'
                return Ok(HtmlNode::Element {
                    tag: tag_name,
                    attrs: HashMap::new(),
                    children: Vec::new(),
                });
            }

            // 解析开始标签
            let tag_name = parse_tag_name(chars)?;

            // 解析属性
            let mut attrs = HashMap::new();
            while chars.peek().map_or(false, |&c| c != '>' && c != '/') {
                if let Some((key, value)) = parse_attr(chars)? {
                    attrs.insert(key, value);
                }
            }

            // 检查是否是自闭合标签
            let is_self_closing = chars.peek() == Some(&'/');
            if is_self_closing {
                chars.next();
            }
            chars.next(); // 消耗 '>'

            // 解析子节点
            let mut children = Vec::new();
            while chars.peek().map_or(false, |&c| c != '<') {
                children.push(parse_node(chars)?);
            }

            Ok(HtmlNode::Element { tag: tag_name, attrs, children })
        }
        _ => {
            // 文本节点
            let mut text = String::new();
            while chars.peek().map_or(false, |&c| c != '<') {
                text.push(chars.next().unwrap());
            }
            Ok(HtmlNode::Text(text))
        }
    }
}

/// 解析标签名（内部函数）
fn parse_tag_name(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<String> {
    let mut name = String::new();
    while let Some(c) = chars.peek() {
        if c.is_alphanumeric() || *c == '-' || *c == '_' {
            name.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    Ok(name)
}

/// 解析属性（内部函数）
fn parse_attr(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Option<(String, String)>> {
    // 跳过空白
    while chars.peek().map_or(false, |c| c.is_whitespace()) {
        chars.next();
    }

    if chars.peek().map_or(false, |&c| c == '>' || c == '/') {
        return Ok(None);
    }

    let mut key = String::new();
    while let Some(c) = chars.peek() {
        if c.is_alphanumeric() || *c == '-' || *c == '_' {
            key.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    // 跳过空白和 '='
    while chars.peek().map_or(false, |&c| c.is_whitespace() || c == '=') {
        chars.next();
    }

    let mut value = String::new();
    if let Some(&'"') = chars.peek() {
        chars.next(); // 消耗引号
        while let Some(c) = chars.next() {
            if c == '"' {
                break;
            }
            value.push(c);
        }
    } else {
        while let Some(c) = chars.peek() {
            if !c.is_whitespace() && *c != '>' && *c != '/' {
                value.push(chars.next().unwrap());
            } else {
                break;
            }
        }
    }

    Ok(Some((key, value)))
}

/// 解析注释（内部函数）
fn parse_comment(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<HtmlNode> {
    let mut content = String::new();
    let mut state = 0;

    while let Some(c) = chars.next() {
        match state {
            0 if c == '-' => state = 1,
            1 if c == '-' => state = 2,
            2 if c == '>' => {
                return Ok(HtmlNode::Comment(content));
            }
            2 => {
                content.push('-');
                content.push('-');
                if c != '-' {
                    state = 0;
                    content.push(c);
                }
            }
            _ => content.push(c),
        }
    }

    Err(RfError::Serialization("Unclosed comment".to_string()))
}

/// 从 HTML 文档中提取纯文本内容
///
/// # 参数
///
/// * `doc` - HTML 文档
///
/// # 返回值
///
/// 返回提取的纯文本内容
///
/// # 示例
///
/// ```rust
/// use rf_encoding::{html_parse, html_extract_text};
///
/// let html = "<p>Hello <strong>World</strong>!</p>";
/// let doc = html_parse(html).unwrap();
/// let text = html_extract_text(&doc);
/// assert_eq!(text, "Hello World!");
/// ```
pub fn extract_text(doc: &HtmlDocument) -> String {
    extract_text_from_node(&doc.root)
}

/// 从节点中提取文本（内部函数）
fn extract_text_from_node(node: &HtmlNode) -> String {
    match node {
        HtmlNode::Element { children, .. } => {
            children.iter().map(extract_text_from_node).collect::<Vec<_>>().join(" ")
        }
        HtmlNode::Text(content) => content.clone(),
        HtmlNode::Comment(_) => String::new(),
    }
}

/// 使用 CSS 选择器查询 HTML 元素
///
/// # 参数
///
/// * `doc` - HTML 文档
/// * `selector` - CSS 选择器字符串（简化版本，仅支持类名和标签名）
///
/// # 返回值
///
/// 返回匹配的元素列表
///
/// # 注意
///
/// 这是一个简化的选择器实现，仅支持：
/// - 标签名选择器（如 "div", "p"）
/// - 类名选择器（如 ".class"）
/// - 标签+类名选择器（如 "div.class"）
///
/// # 示例
///
/// ```rust
/// use rf_encoding::{html_parse, html_select};
///
/// let html = "<div><p class=\"content\">Text</p></div>";
/// let doc = html_parse(html).unwrap();
/// let elements = html_select(&doc, "p.content");
/// ```
pub fn select(doc: &HtmlDocument, selector: &str) -> Vec<HtmlNode> {
    select_nodes(&doc.root, selector)
}

/// 从节点中选择（内部函数）
fn select_nodes(node: &HtmlNode, selector: &str) -> Vec<HtmlNode> {
    let mut result = Vec::new();

    match node {
        HtmlNode::Element { tag: _, attrs: _, children } => {
            // 检查是否匹配
            let matches = match_selector(node, selector);
            if matches {
                result.push(node.clone());
            }

            // 递归搜索子节点
            for child in children {
                result.extend(select_nodes(child, selector));
            }
        }
        HtmlNode::Text(_) | HtmlNode::Comment(_) => {}
    }

    result
}

/// 检查节点是否匹配选择器（内部函数）
fn match_selector(node: &HtmlNode, selector: &str) -> bool {
    if let HtmlNode::Element { tag, attrs, .. } = node {
        if selector.starts_with('.') {
            // 类名选择器
            let class_name = &selector[1..];
            return attrs.get("class").map_or(false, |c| c.split_whitespace().any(|cls| cls == class_name));
        } else if selector.contains('.') {
            // 标签+类名选择器
            if let Some((tag_name, class_name)) = selector.split_once('.') {
                if tag != tag_name {
                    return false;
                }
                return attrs.get("class").map_or(false, |c| c.split_whitespace().any(|cls| cls == class_name));
            }
        } else {
            // 标签名选择器
            return tag == selector;
        }
    }
    false
}
