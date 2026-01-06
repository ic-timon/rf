//! # fpool
//!
//! fpool 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! File pool for managing file handles

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use rf_errors::Result;

/// File pool for managing file handles
pub struct FilePool {
    base_path: PathBuf,
    files: Arc<Mutex<std::collections::HashMap<String, Arc<Mutex<File>>>>>,
}

impl FilePool {
    /// Create a new file pool
    pub fn new(base_path: &str) -> Result<Self> {
        std::fs::create_dir_all(base_path)
            .map_err(rf_errors::RfError::Io)?;
        Ok(Self {
            base_path: PathBuf::from(base_path),
            files: Arc::new(Mutex::new(std::collections::HashMap::new())),
        })
    }

    /// Get or create a file handle
    pub async fn get(&self, name: &str) -> Result<Arc<Mutex<File>>> {
        let mut files = self.files.lock().await;
        if let Some(file) = files.get(name) {
            return Ok(file.clone());
        }
        
        let path = self.base_path.join(name);
        let file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&path)
            .map_err(rf_errors::RfError::Io)?;
        let file = Arc::new(Mutex::new(file));
        files.insert(name.to_string(), file.clone());
        Ok(file)
    }

    /// Remove a file from the pool
    pub async fn remove(&self, name: &str) -> Result<()> {
        let mut files = self.files.lock().await;
        files.remove(name);
        Ok(())
    }

    /// Read from a file
    pub async fn read(&self, name: &str, buf: &mut [u8]) -> Result<usize> {
        let file = self.get(name).await?;
        let mut file = file.lock().await;
        file.read(buf)
            .map_err(rf_errors::RfError::Io)
    }

    /// Write to a file
    pub async fn write(&self, name: &str, buf: &[u8]) -> Result<usize> {
        let file = self.get(name).await?;
        let mut file = file.lock().await;
        file.write(buf)
            .map_err(rf_errors::RfError::Io)
    }
}
