//! # file_test
//!
//! file_test 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! File operation tests

#[cfg(test)]
mod tests {
    use rf_os::file::*;
    use tempfile::TempDir;

    #[test]
    fn test_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        std::fs::write(&file_path, "test").unwrap();
        
        assert!(exists(file_path.to_str().unwrap()));
    }

    #[test]
    fn test_file_read_write() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let content = "Hello, World!";
        
        write_string(file_path.to_str().unwrap(), content).unwrap();
        let read_content = read_string(file_path.to_str().unwrap()).unwrap();
        
        assert_eq!(read_content, content);
    }
}

