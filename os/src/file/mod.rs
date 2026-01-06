//! # file 模块
//!
//! 文件操作模块，提供文件和目录的基本操作功能。
//!
//! ## 主要功能
//!
//! - 文件读写操作
//! - 目录管理
//! - 路径操作
//! - 文件搜索
//! - 文件信息获取
//!
//! @author TimonQWQ
//! @date 2026-01-06

// ========== 子模块声明 ==========

pub mod advanced;

// 导出子模块的公共接口
pub use advanced::*;

use rf_errors::Result;
use std::fs;
use std::path::{Path, PathBuf};

// ========== 基本文件操作 ==========

/// 检查路径是否存在
///
/// # 参数
///
/// - `path`: 文件或目录路径
///
/// # 返回值
///
/// 如果路径存在返回 `true`，否则返回 `false`
///
/// # 示例
///
/// ```rust
/// if exists("/path/to/file") {
///     println!("文件存在");
/// }
/// ```
pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// 检查路径是否为文件
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 如果是文件返回 `true`，否则返回 `false`
pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}

/// 检查路径是否为目录
///
/// # 参数
///
/// - `path`: 目录路径
///
/// # 返回值
///
/// 如果是目录返回 `true`，否则返回 `false`
pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

/// 读取文件内容为字符串
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 返回 `Result<String>`，包含文件内容
///
/// # 错误
///
/// 如果文件不存在或读取失败，返回错误
///
/// # 示例
///
/// ```rust
/// let content = read_string("/path/to/file.txt")?;
/// ```
pub fn read_string(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .map_err(rf_errors::RfError::Io)
}

/// 读取文件内容为字节
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 返回 `Result<Vec<u8>>`，包含文件内容
pub fn read_bytes(path: &str) -> Result<Vec<u8>> {
    fs::read(path)
        .map_err(rf_errors::RfError::Io)
}

/// 写入字符串到文件
///
/// # 参数
///
/// - `path`: 文件路径
/// - `content`: 要写入的内容
///
/// # 返回值
///
/// 返回 `Result<()>`
///
/// # 注意
///
/// 如果文件已存在，会覆盖原文件内容
pub fn write_string(path: &str, content: &str) -> Result<()> {
    fs::write(path, content)
        .map_err(rf_errors::RfError::Io)
}

/// 写入字节到文件
///
/// # 参数
///
/// - `path`: 文件路径
/// - `content`: 要写入的字节
///
/// # 返回值
///
/// 返回 `Result<()>`
pub fn write_bytes(path: &str, content: &[u8]) -> Result<()> {
    fs::write(path, content)
        .map_err(rf_errors::RfError::Io)
}

// ========== 目录操作 ==========

/// 创建目录
///
/// # 参数
///
/// - `path`: 目录路径
///
/// # 返回值
///
/// 返回 `Result<()>`
///
/// # 注意
///
/// 如果父目录不存在，会自动创建
pub fn create_dir(path: &str) -> Result<()> {
    fs::create_dir_all(path)
        .map_err(rf_errors::RfError::Io)
}

/// 删除目录
///
/// # 参数
///
/// - `path`: 目录路径
///
/// # 返回值
///
/// 返回 `Result<()>`
///
/// # 注意
///
/// 会递归删除目录及其所有内容
pub fn remove_dir(path: &str) -> Result<()> {
    fs::remove_dir_all(path)
        .map_err(rf_errors::RfError::Io)
}

// ========== 文件操作 ==========

/// 删除文件
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 返回 `Result<()>`
pub fn remove_file(path: &str) -> Result<()> {
    fs::remove_file(path)
        .map_err(rf_errors::RfError::Io)
}

/// 复制文件
///
/// # 参数
///
/// - `src`: 源文件路径
/// - `dst`: 目标文件路径
///
/// # 返回值
///
/// 返回 `Result<u64>`，包含复制的字节数
pub fn copy(src: &str, dst: &str) -> Result<u64> {
    fs::copy(src, dst)
        .map_err(rf_errors::RfError::Io)
}

/// 移动/重命名文件或目录
///
/// # 参数
///
/// - `src`: 源路径
/// - `dst`: 目标路径
///
/// # 返回值
///
/// 返回 `Result<()>`
pub fn r#move(src: &str, dst: &str) -> Result<()> {
    fs::rename(src, dst)
        .map_err(rf_errors::RfError::Io)
}

/// 重命名文件或目录
///
/// # 参数
///
/// - `src`: 源路径
/// - `dst`: 目标路径
///
/// # 返回值
///
/// 返回 `Result<()>`
pub fn rename(src: &str, dst: &str) -> Result<()> {
    r#move(src, dst)
}

// ========== 文件信息 ==========

/// 获取文件大小
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 返回 `Result<u64>`，包含文件大小（字节）
pub fn size(path: &str) -> Result<u64> {
    fs::metadata(path)
        .map(|m| m.len())
        .map_err(rf_errors::RfError::Io)
}

/// 获取文件修改时间
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 返回 `Result<DateTime<Utc>>`，包含最后修改时间
pub fn modified_time(path: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    let metadata = fs::metadata(path)
        .map_err(rf_errors::RfError::Io)?;
    let modified = metadata.modified()
        .map_err(rf_errors::RfError::Io)?;
    Ok(chrono::DateTime::<chrono::Utc>::from(modified))
}

/// 检查文件是否可读
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 如果可读返回 `true`，否则返回 `false`
pub fn is_readable(path: &str) -> bool {
    fs::File::open(path).is_ok()
}

/// 检查文件是否可写
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 如果可写返回 `true`，否则返回 `false`
pub fn is_writable(path: &str) -> bool {
    if fs::OpenOptions::new().write(true).open(path).is_ok() {
        true
    } else {
        // 尝试在同一目录创建临时文件
        if let Some(parent) = Path::new(path).parent() {
            if tempfile::NamedTempFile::new_in(parent).is_ok() {
                return true;
            }
        }
        false
    }
}

// ========== 路径操作 ==========

/// 获取文件扩展名
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 返回文件扩展名（不含点号），如果没有扩展名返回空字符串
///
/// # 示例
///
/// ```rust
/// assert_eq!(ext("file.txt"), "txt");
/// assert_eq!(ext("file.tar.gz"), "gz");
/// ```
pub fn ext(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string()
}

/// 获取文件名（不含扩展名）
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 返回文件名（不含扩展名）
pub fn name(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string()
}

/// 获取文件名（含扩展名）
///
/// # 参数
///
/// - `path`: 文件路径
///
/// # 返回值
///
/// 返回完整文件名
pub fn basename(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string()
}

/// 获取目录名
///
/// # 参数
///
/// - `path`: 文件或目录路径
///
/// # 返回值
///
/// 返回父目录路径
pub fn dirname(path: &str) -> String {
    Path::new(path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("")
        .to_string()
}

/// 连接多个路径
///
/// # 参数
///
/// - `paths`: 路径片段数组
///
/// # 返回值
///
/// 返回连接后的路径字符串
///
/// # 示例
///
/// ```rust
/// assert_eq!(join(&["a", "b", "c"]), "a/b/c"); // Unix
/// ```
pub fn join(paths: &[&str]) -> String {
    let mut result = PathBuf::new();
    for path in paths {
        result.push(path);
    }
    result.to_string_lossy().to_string()
}

/// 获取绝对路径
///
/// # 参数
///
/// - `path`: 相对或绝对路径
///
/// # 返回值
///
/// 返回 `Result<String>`，包含规范化后的绝对路径
pub fn abs(path: &str) -> Result<String> {
    let abs_path = fs::canonicalize(path)
        .map_err(rf_errors::RfError::Io)?;
    Ok(abs_path.to_string_lossy().to_string())
}

// ========== 文件搜索 ==========

/// 按正则表达式搜索文件
///
/// # 参数
///
/// - `dir`: 搜索目录
/// - `pattern`: 正则表达式模式
///
/// # 返回值
///
/// 返回 `Result<Vec<String>>`，包含匹配的文件路径
pub fn search(dir: &str, pattern: &str) -> Result<Vec<String>> {
    use walkdir::WalkDir;
    let re = regex::Regex::new(pattern)
        .map_err(|e| rf_errors::RfError::Internal(format!("Invalid regex pattern: {}", e)))?;
    let mut results = Vec::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.map_err(|e| rf_errors::RfError::Io(std::io::Error::other(format!("{}", e))))?;
        let path = entry.path().to_string_lossy().to_string();
        if re.is_match(&path) {
            results.push(path);
        }
    }
    Ok(results)
}

/// 扫描目录
///
/// # 参数
///
/// - `dir`: 目录路径
///
/// # 返回值
///
/// 返回 `Result<Vec<String>>`，包含所有文件和子目录的路径
pub fn scan(dir: &str) -> Result<Vec<String>> {
    let mut results = Vec::new();
    let entries = fs::read_dir(dir)
        .map_err(rf_errors::RfError::Io)?;
    for entry in entries {
        let entry = entry.map_err(rf_errors::RfError::Io)?;
        let path = entry.path().to_string_lossy().to_string();
        results.push(path);
    }
    Ok(results)
}

/// 获取目录下的子目录名
///
/// # 参数
///
/// - `dir`: 目录路径
///
/// # 返回值
///
/// 返回 `Result<Vec<String>>`，包含所有子目录的名称
pub fn dir_names(dir: &str) -> Result<Vec<String>> {
    let mut results = Vec::new();
    let entries = fs::read_dir(dir)
        .map_err(rf_errors::RfError::Io)?;
    for entry in entries {
        let entry = entry.map_err(rf_errors::RfError::Io)?;
        if entry.path().is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                results.push(name.to_string());
            }
        }
    }
    Ok(results)
}

// ========== 文件内容操作 ==========

/// 替换文件内容
///
/// # 参数
///
/// - `path`: 文件路径
/// - `old`: 要查找的文本
/// - `new`: 替换后的文本
///
/// # 返回值
///
/// 返回 `Result<()>`
pub fn replace(path: &str, old: &str, new: &str) -> Result<()> {
    let content = read_string(path)?;
    let new_content = content.replace(old, new);
    write_string(path, &new_content)
}

/// 格式化文件大小
///
/// # 参数
///
/// - `size`: 文件大小（字节）
///
/// # 返回值
///
/// 返回格式化后的字符串（如 "1.23 MB"）
pub fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    format!("{:.2} {}", size, UNITS[unit_index])
}

// ========== 项目路径 ==========

/// 获取主包路径（Cargo 项目根目录）
///
/// 从当前目录向上查找 Cargo.toml 或 Cargo.lock 文件。
/// 支持常规项目和工作空间项目。
///
/// # 返回值
///
/// 返回 `Result<String>`，包含项目根目录的绝对路径
pub fn main_pkg_path() -> Result<String> {
    // 从当前目录开始
    let mut current = std::env::current_dir()
        .map_err(rf_errors::RfError::Io)?;

    loop {
        // 检查 Cargo.toml
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            // 检查是否为工作空间
            if let Ok(content) = fs::read_to_string(&cargo_toml) {
                // 如果包含 [workspace]，则是工作空间根目录
                if content.contains("[workspace]") {
                    return Ok(current.to_string_lossy().to_string());
                }
                // 否则是常规包
                return Ok(current.to_string_lossy().to_string());
            }
        }

        // 检查 Cargo.lock（也指示项目根目录）
        let cargo_lock = current.join("Cargo.lock");
        if cargo_lock.exists() {
            return Ok(current.to_string_lossy().to_string());
        }

        // 移动到父目录
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => break, // 到达文件系统根目录
        }
    }

    // 回退：返回当前目录
    std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(rf_errors::RfError::Io)
}

/// 获取可执行文件路径
///
/// # 返回值
///
/// 返回 `Result<String>`，包含当前可执行文件的绝对路径
pub fn executable_path() -> Result<String> {
    std::env::current_exe()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(rf_errors::RfError::Io)
}

// ========== 临时文件 ==========

/// 创建临时文件
///
/// # 参数
///
/// - `prefix`: 文件名前缀
/// - `suffix`: 文件名后缀（扩展名）
///
/// # 返回值
///
/// 返回 `Result<String>`，包含临时文件的路径
pub fn temp_file(prefix: &str, suffix: &str) -> Result<String> {
    let mut builder = tempfile::Builder::new();
    builder.prefix(prefix);
    builder.suffix(suffix);
    let file = builder.tempfile()
        .map_err(rf_errors::RfError::Io)?;
    Ok(file.path().to_string_lossy().to_string())
}

/// 创建临时目录
///
/// # 参数
///
/// - `prefix`: 目录名前缀
///
/// # 返回值
///
/// 返回 `Result<String>`，包含临时目录的路径
pub fn temp_dir(prefix: &str) -> Result<String> {
    let dir = tempfile::Builder::new()
        .prefix(prefix)
        .tempdir()
        .map_err(rf_errors::RfError::Io)?;
    Ok(dir.path().to_string_lossy().to_string())
}
