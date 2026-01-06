//! # advanced
//!
//! advanced 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Advanced file operations: matching, sorting, caching, permissions, content search

use rf_errors::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use regex::Regex;
use chrono::DateTime;
use chrono::Utc;
use moka::future::Cache;
use std::time::Duration;

/// File metadata cache
pub struct FileMetadataCache {
    cache: Arc<Cache<String, FileMetadata>>,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub is_dir: bool,
    pub is_file: bool,
}

impl FileMetadataCache {
    /// Create a new file metadata cache
    pub fn new(capacity: u64, ttl: Duration) -> Self {
        let cache = Cache::builder()
            .max_capacity(capacity)
            .time_to_live(ttl)
            .build();
        
        Self {
            cache: Arc::new(cache),
        }
    }

    /// Get cached metadata
    pub async fn get(&self, path: &str) -> Option<FileMetadata> {
        self.cache.get(path).await
    }

    /// Set cached metadata
    pub async fn set(&self, path: String, metadata: FileMetadata) {
        self.cache.insert(path, metadata).await;
    }

    /// Invalidate cache entry
    pub async fn invalidate(&self, path: &str) {
        self.cache.invalidate(path).await;
    }

    /// Clear all cache
    pub fn clear(&self) {
        self.cache.invalidate_all();
    }
}

impl Default for FileMetadataCache {
    fn default() -> Self {
        Self::new(1000, Duration::from_secs(300))
    }
}

/// Get file metadata (with caching)
pub async fn get_metadata_cached(path: &str, cache: &FileMetadataCache) -> Result<FileMetadata> {
    // Check cache first
    if let Some(metadata) = cache.get(path).await {
        return Ok(metadata);
    }

    // Get metadata from filesystem
    let metadata = fs::metadata(path)
        .map_err(rf_errors::RfError::Io)?;
    
    let file_metadata = FileMetadata {
        path: path.to_string(),
        size: metadata.len(),
        modified: DateTime::<Utc>::from(metadata.modified().map_err(rf_errors::RfError::Io)?),
        is_dir: metadata.is_dir(),
        is_file: metadata.is_file(),
    };

    // Cache the result
    cache.set(path.to_string(), file_metadata.clone()).await;

    Ok(file_metadata)
}

/// File matching patterns
pub enum MatchPattern {
    Glob(String),
    Regex(Regex),
    Extension(String),
    Name(String),
}

/// Match files by pattern
pub fn match_files(dir: &str, pattern: &MatchPattern) -> Result<Vec<PathBuf>> {
    let mut matches = Vec::new();
    
    if !Path::new(dir).exists() {
        return Ok(matches);
    }

    let entries = walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in entries {
        let path = entry.path();
        let matched = match pattern {
            MatchPattern::Glob(glob_pattern) => {
                // Simple glob matching (simplified)
                path.to_string_lossy().contains(glob_pattern)
            }
            MatchPattern::Regex(regex) => {
                regex.is_match(path.to_string_lossy().as_ref())
            }
            MatchPattern::Extension(ext) => {
                path.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e == ext)
                    .unwrap_or(false)
            }
            MatchPattern::Name(name) => {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n == name)
                    .unwrap_or(false)
            }
        };

        if matched {
            matches.push(path.to_path_buf());
        }
    }

    Ok(matches)
}

/// Sort order for files
pub enum SortOrder {
    NameAsc,
    NameDesc,
    SizeAsc,
    SizeDesc,
    ModifiedAsc,
    ModifiedDesc,
}

/// Sort files by order
pub fn sort_files(files: &mut [PathBuf], order: SortOrder) -> Result<()> {
    match order {
        SortOrder::NameAsc => {
            files.sort_by(|a, b| {
                a.file_name().cmp(&b.file_name())
            });
        }
        SortOrder::NameDesc => {
            files.sort_by(|a, b| {
                b.file_name().cmp(&a.file_name())
            });
        }
        SortOrder::SizeAsc => {
            files.sort_by(|a, b| {
                let size_a = fs::metadata(a).map(|m| m.len()).unwrap_or(0);
                let size_b = fs::metadata(b).map(|m| m.len()).unwrap_or(0);
                size_a.cmp(&size_b)
            });
        }
        SortOrder::SizeDesc => {
            files.sort_by(|a, b| {
                let size_a = fs::metadata(a).map(|m| m.len()).unwrap_or(0);
                let size_b = fs::metadata(b).map(|m| m.len()).unwrap_or(0);
                size_b.cmp(&size_a)
            });
        }
        SortOrder::ModifiedAsc => {
            files.sort_by(|a, b| {
                let time_a = fs::metadata(a)
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::UNIX_EPOCH);
                let time_b = fs::metadata(b)
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::UNIX_EPOCH);
                time_a.cmp(&time_b)
            });
        }
        SortOrder::ModifiedDesc => {
            files.sort_by(|a, b| {
                let time_a = fs::metadata(a)
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::UNIX_EPOCH);
                let time_b = fs::metadata(b)
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::UNIX_EPOCH);
                time_b.cmp(&time_a)
            });
        }
    }
    Ok(())
}

/// Change file permissions (chmod)
pub fn chmod(path: &str, mode: u32) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(mode);
        fs::set_permissions(path, perms)
            .map_err(rf_errors::RfError::Io)?;
    }
    #[cfg(not(unix))]
    {
        // On non-Unix systems, permissions are limited
        // This is a placeholder
    }
    Ok(())
}

/// Get file permissions
pub fn get_permissions(path: &str) -> Result<u32> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(path)
            .map_err(rf_errors::RfError::Io)?;
        Ok(metadata.permissions().mode())
    }
    #[cfg(not(unix))]
    {
        // On non-Unix systems, return default
        Ok(0o644)
    }
}

/// Search for text in file content
pub fn search_in_file(path: &str, pattern: &str, case_sensitive: bool) -> Result<Vec<usize>> {
    let content = fs::read_to_string(path)
        .map_err(rf_errors::RfError::Io)?;
    
    let search_text = if case_sensitive {
        pattern.to_string()
    } else {
        pattern.to_lowercase()
    };
    
    let file_content = if case_sensitive {
        content
    } else {
        content.to_lowercase()
    };

    let mut matches = Vec::new();
    let mut start = 0;
    
    while let Some(pos) = file_content[start..].find(&search_text) {
        matches.push(start + pos);
        start += pos + search_text.len();
    }

    Ok(matches)
}

/// Search for text in multiple files
pub fn search_in_files(paths: &[&str], pattern: &str, case_sensitive: bool) -> Result<HashMap<String, Vec<usize>>> {
    let mut results = HashMap::new();
    
    for path in paths {
        if let Ok(matches) = search_in_file(path, pattern, case_sensitive) {
            if !matches.is_empty() {
                results.insert(path.to_string(), matches);
            }
        }
    }
    
    Ok(results)
}

/// Search for text in directory recursively
pub fn search_in_directory(dir: &str, pattern: &str, case_sensitive: bool) -> Result<HashMap<String, Vec<usize>>> {
    let mut results = HashMap::new();
    
    let entries = walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file());

    for entry in entries {
        let path = entry.path().to_string_lossy().to_string();
        if let Ok(matches) = search_in_file(&path, pattern, case_sensitive) {
            if !matches.is_empty() {
                results.insert(path, matches);
            }
        }
    }
    
    Ok(results)
}

