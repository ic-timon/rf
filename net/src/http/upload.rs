//! # upload
//!
//! upload 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! HTTP file upload handling

use axum::extract::Multipart;
use rf_errors::Result;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Uploaded file information
#[derive(Debug, Clone)]
pub struct UploadFile {
    pub field_name: String,
    pub filename: String,
    pub content_type: Option<String>,
    pub size: u64,
    pub data: Vec<u8>,
}

impl UploadFile {
    /// Create a new upload file
    pub fn new(field_name: String, filename: String, content_type: Option<String>, data: Vec<u8>) -> Self {
        let size = data.len() as u64;
        Self {
            field_name,
            filename,
            content_type,
            size,
            data,
        }
    }

    /// Get the file extension
    pub fn extension(&self) -> Option<&str> {
        Path::new(&self.filename)
            .extension()
            .and_then(|ext| ext.to_str())
    }

    /// Save the file to the specified directory
    pub async fn save(&self, dir_path: impl AsRef<Path>) -> Result<String> {
        let dir_path = dir_path.as_ref();
        
        // Create directory if it doesn't exist
        if !dir_path.exists() {
            fs::create_dir_all(dir_path).await
                .map_err(rf_errors::RfError::Io)?;
        }
        
        let file_path = dir_path.join(&self.filename);
        let mut file = fs::File::create(&file_path).await
            .map_err(rf_errors::RfError::Io)?;
        
        file.write_all(&self.data).await
            .map_err(rf_errors::RfError::Io)?;
        
        file.flush().await
            .map_err(rf_errors::RfError::Io)?;
        
        Ok(self.filename.clone())
    }

    /// Save the file with a custom filename
    pub async fn save_as(&self, file_path: impl AsRef<Path>) -> Result<()> {
        let file_path = file_path.as_ref();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await
                    .map_err(rf_errors::RfError::Io)?;
            }
        }
        
        let mut file = fs::File::create(file_path).await
            .map_err(rf_errors::RfError::Io)?;
        
        file.write_all(&self.data).await
            .map_err(rf_errors::RfError::Io)?;
        
        file.flush().await
            .map_err(rf_errors::RfError::Io)?;
        
        Ok(())
    }
}

/// File upload validator
pub struct FileValidator {
    max_size: Option<u64>,
    allowed_extensions: Option<Vec<String>>,
    allowed_content_types: Option<Vec<String>>,
}

impl FileValidator {
    /// Create a new file validator
    pub fn new() -> Self {
        Self {
            max_size: None,
            allowed_extensions: None,
            allowed_content_types: None,
        }
    }

    /// Set maximum file size in bytes
    pub fn max_size(mut self, size: u64) -> Self {
        self.max_size = Some(size);
        self
    }

    /// Set allowed file extensions
    pub fn allowed_extensions(mut self, extensions: Vec<String>) -> Self {
        self.allowed_extensions = Some(extensions);
        self
    }

    /// Set allowed content types
    pub fn allowed_content_types(mut self, content_types: Vec<String>) -> Self {
        self.allowed_content_types = Some(content_types);
        self
    }

    /// Validate a file
    pub fn validate(&self, file: &UploadFile) -> Result<()> {
        // Check file size
        if let Some(max_size) = self.max_size {
            if file.size > max_size {
                return Err(rf_errors::RfError::Validation(
                    format!("File size {} exceeds maximum allowed size {}", file.size, max_size)
                ));
            }
        }

        // Check file extension
        if let Some(ref allowed_exts) = self.allowed_extensions {
            if let Some(ext) = file.extension() {
                let ext_lower = ext.to_lowercase();
                if !allowed_exts.iter().any(|e| e.to_lowercase() == ext_lower) {
                    return Err(rf_errors::RfError::Validation(
                        format!("File extension '{}' is not allowed", ext)
                    ));
                }
            } else {
                return Err(rf_errors::RfError::Validation(
                    "File must have an extension".to_string()
                ));
            }
        }

        // Check content type
        if let Some(ref allowed_types) = self.allowed_content_types {
            if let Some(ref content_type) = file.content_type {
                if !allowed_types.iter().any(|t| t == content_type) {
                    return Err(rf_errors::RfError::Validation(
                        format!("Content type '{}' is not allowed", content_type)
                    ));
                }
            }
        }

        Ok(())
    }
}

impl Default for FileValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract uploaded files from multipart form data
pub async fn extract_files(mut multipart: Multipart) -> Result<Vec<UploadFile>> {
    let mut files = Vec::new();

    while let Some(field) = multipart.next_field().await
        .map_err(|e| rf_errors::RfError::Network(format!("Failed to read multipart field: {}", e)))? {
        
        let field_name = field.name()
            .ok_or_else(|| rf_errors::RfError::Network("Field name is missing".to_string()))?
            .to_string();
        
        let filename = field.file_name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("unknown_{}", field_name));
        
        let content_type = field.content_type().map(|s| s.to_string());
        
        let data = field.bytes().await
            .map_err(|e| rf_errors::RfError::Network(format!("Failed to read field data: {}", e)))?;
        
        let file = UploadFile::new(
            field_name,
            filename,
            content_type,
            data.to_vec(),
        );
        
        files.push(file);
    }

    Ok(files)
}

/// Extract a single uploaded file by field name
pub async fn extract_file(mut multipart: Multipart, field_name: &str) -> Result<Option<UploadFile>> {
    while let Some(field) = multipart.next_field().await
        .map_err(|e| rf_errors::RfError::Network(format!("Failed to read multipart field: {}", e)))? {
        
        if let Some(name) = field.name() {
            if name == field_name {
                let filename = field.file_name()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("unknown_{}", field_name));
                
                let content_type = field.content_type().map(|s| s.to_string());
                
                let data = field.bytes().await
                    .map_err(|e| rf_errors::RfError::Network(format!("Failed to read field data: {}", e)))?;
                
                let file = UploadFile::new(
                    field_name.to_string(),
                    filename,
                    content_type,
                    data.to_vec(),
                );
                
                return Ok(Some(file));
            }
        }
    }

    Ok(None)
}

/// Extract multiple uploaded files by field name
pub async fn extract_files_by_name(mut multipart: Multipart, field_name: &str) -> Result<Vec<UploadFile>> {
    let mut files = Vec::new();

    while let Some(field) = multipart.next_field().await
        .map_err(|e| rf_errors::RfError::Network(format!("Failed to read multipart field: {}", e)))? {
        
        if let Some(name) = field.name() {
            if name == field_name {
                let filename = field.file_name()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("unknown_{}", field_name));
                
                let content_type = field.content_type().map(|s| s.to_string());
                
                let data = field.bytes().await
                    .map_err(|e| rf_errors::RfError::Network(format!("Failed to read field data: {}", e)))?;
                
                let file = UploadFile::new(
                    field_name.to_string(),
                    filename,
                    content_type,
                    data.to_vec(),
                );
                
                files.push(file);
            }
        }
    }

    Ok(files)
}

