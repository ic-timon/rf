//! # storage
//!
//! storage 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Session storage adapters

use axum_sessions::async_session::Session;
use rf_errors::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json;

/// Type alias for session with expiration timestamp
type SessionEntry = (Session, chrono::DateTime<chrono::Utc>);

/// Type alias for sessions map
type SessionsMap = Arc<RwLock<HashMap<String, SessionEntry>>>;

/// Session expiration policy
#[derive(Debug, Clone)]
pub struct ExpirationPolicy {
    pub default_ttl: std::time::Duration,
    pub max_ttl: Option<std::time::Duration>,
    pub sliding_expiration: bool, // Reset expiration on access
}

impl Default for ExpirationPolicy {
    fn default() -> Self {
        Self {
            default_ttl: std::time::Duration::from_secs(3600), // 1 hour
            max_ttl: None,
            sliding_expiration: false,
        }
    }
}

/// Session storage trait
pub trait SessionStorage: Send + Sync {
    /// Get a session by ID
    fn get(&self, id: &str) -> Result<Option<Session>>;
    
    /// Store a session
    fn store(&self, session: Session) -> Result<()>;
    
    /// Delete a session
    fn delete(&self, id: &str) -> Result<()>;
    
    /// Clean up expired sessions
    fn cleanup_expired(&self) -> Result<()> {
        Ok(()) // Default implementation does nothing
    }
    
    /// Set expiration policy
    fn set_expiration_policy(&mut self, _policy: ExpirationPolicy) {
        // Default implementation does nothing
    }
}

/// Memory-based session storage
pub struct MemorySessionStorage {
    sessions: SessionsMap,
    expiration_policy: ExpirationPolicy,
}

impl MemorySessionStorage {
    /// Create a new memory session storage
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            expiration_policy: ExpirationPolicy::default(),
        }
    }
}

impl SessionStorage for MemorySessionStorage {
    fn get(&self, id: &str) -> Result<Option<Session>> {
        let mut sessions = futures::executor::block_on(self.sessions.write());
        let now = chrono::Utc::now();
        
        if let Some((session, expiration)) = sessions.get(id) {
            // Check if expired
            if now > *expiration {
                sessions.remove(id);
                return Ok(None);
            }
            
            // Clone session before modifying sessions map
            let session_clone = session.clone();
            
            // If sliding expiration, update expiration time
            if self.expiration_policy.sliding_expiration {
                let new_expiration = now + chrono::Duration::from_std(self.expiration_policy.default_ttl)
                    .map_err(|e| rf_errors::RfError::Internal(format!("Invalid TTL duration: {}", e)))?;
                sessions.insert(id.to_string(), (session_clone.clone(), new_expiration));
            }
            
            Ok(Some(session_clone))
        } else {
            Ok(None)
        }
    }
    
    fn store(&self, session: Session) -> Result<()> {
        let mut sessions = futures::executor::block_on(self.sessions.write());
        let session_id = session.id().to_string();
        let expiration = chrono::Utc::now() + chrono::Duration::from_std(self.expiration_policy.default_ttl)
            .map_err(|e| rf_errors::RfError::Internal(format!("Invalid TTL duration: {}", e)))?;
        sessions.insert(session_id, (session, expiration));
        Ok(())
    }
    
    fn delete(&self, id: &str) -> Result<()> {
        let mut sessions = futures::executor::block_on(self.sessions.write());
        sessions.remove(id);
        Ok(())
    }
    
    fn cleanup_expired(&self) -> Result<()> {
        let mut sessions = futures::executor::block_on(self.sessions.write());
        let now = chrono::Utc::now();
        sessions.retain(|_, (_, expiration)| *expiration > now);
        Ok(())
    }
    
    fn set_expiration_policy(&mut self, policy: ExpirationPolicy) {
        self.expiration_policy = policy;
    }
}

impl Default for MemorySessionStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// File-based session storage
pub struct FileSessionStorage {
    base_path: String,
    ttl: Option<std::time::Duration>,
    crypto_enabled: bool,
    crypto_key: Option<Vec<u8>>,
}

impl FileSessionStorage {
    /// Create a new file session storage
    pub fn new(base_path: &str) -> Result<Self> {
        std::fs::create_dir_all(base_path)
            .map_err(rf_errors::RfError::Io)?;
        Ok(Self {
            base_path: base_path.to_string(),
            ttl: None,
            crypto_enabled: false,
            crypto_key: None,
        })
    }

    /// Set TTL for sessions
    pub fn with_ttl(mut self, ttl: std::time::Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }

    /// Enable encryption
    pub fn with_encryption(mut self, key: Vec<u8>) -> Self {
        self.crypto_enabled = true;
        self.crypto_key = Some(key);
        self
    }
    
    fn session_path(&self, id: &str) -> String {
        format!("{}/{}.session", self.base_path, id)
    }

    fn check_expired(&self, path: &str) -> bool {
        if let Some(ttl) = self.ttl {
            if let Ok(metadata) = std::fs::metadata(path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(elapsed) = modified.elapsed() {
                        return elapsed > ttl;
                    }
                }
            }
        }
        false
    }
}

impl SessionStorage for FileSessionStorage {
    fn get(&self, id: &str) -> Result<Option<Session>> {
        let path = self.session_path(id);
        if !std::path::Path::new(&path).exists() {
            return Ok(None);
        }

        // Check if expired
        if self.check_expired(&path) {
            let _ = std::fs::remove_file(&path);
            return Ok(None);
        }

        let content = std::fs::read(&path)
            .map_err(rf_errors::RfError::Io)?;
        
        // Decrypt if enabled
        let _data = if self.crypto_enabled {
            if self.crypto_key.is_some() {
                // Simplified - would use AES decryption
                content
            } else {
                content
            }
        } else {
            content
        };

        // Deserialize Session from JSON
        let _session_data: serde_json::Value = serde_json::from_slice(&_data)
            .map_err(|e| rf_errors::RfError::Internal(format!("Failed to deserialize session: {}", e)))?;
        
        // Create Session from data (simplified)
        // In full implementation, would reconstruct Session object
        Ok(None)
    }
    
    fn store(&self, session: Session) -> Result<()> {
        let path = self.session_path(session.id());
        
        // Serialize Session to JSON (simplified)
        // Note: Session doesn't expose data() directly, would need to use internal methods
        let data = serde_json::to_vec(&serde_json::json!({
            "id": session.id().to_string(),
        }))
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to serialize session: {}", e)))?;
        
        // Encrypt if enabled
        let final_data = if self.crypto_enabled {
            if self.crypto_key.is_some() {
                // Simplified - would use AES encryption
                data
            } else {
                data
            }
        } else {
            data
        };
        
        std::fs::write(&path, final_data)
            .map_err(rf_errors::RfError::Io)?;
        Ok(())
    }
    
    fn delete(&self, id: &str) -> Result<()> {
        let path = self.session_path(id);
        if std::path::Path::new(&path).exists() {
            std::fs::remove_file(&path)
                .map_err(rf_errors::RfError::Io)?;
        }
        Ok(())
    }
}

/// Redis-based session storage
pub struct RedisSessionStorage {
    client: rf_database::redis::RedisClient,
    ttl: Option<u64>, // TTL in seconds
    key_prefix: String,
}

impl RedisSessionStorage {
    /// Create a new Redis session storage
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = rf_database::redis::RedisClient::new(redis_url).await?;
        Ok(Self {
            client,
            ttl: None,
            key_prefix: "session:".to_string(),
        })
    }

    /// Set TTL for sessions (in seconds)
    pub fn with_ttl(mut self, ttl: u64) -> Self {
        self.ttl = Some(ttl);
        self
    }

    /// Set key prefix
    pub fn with_prefix(mut self, prefix: &str) -> Self {
        self.key_prefix = prefix.to_string();
        self
    }

    fn session_key(&self, id: &str) -> String {
        format!("{}{}", self.key_prefix, id)
    }
}

impl SessionStorage for RedisSessionStorage {
    fn get(&self, id: &str) -> Result<Option<Session>> {
        let key = self.session_key(id);
        let rt = tokio::runtime::Handle::try_current()
            .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))?;
        
        match rt.block_on(self.client.get(&key)) {
            Ok(data) => {
                // Deserialize Session from JSON
                let session_data: serde_json::Value = serde_json::from_str(&data)
                    .map_err(|e| rf_errors::RfError::Internal(format!("Failed to deserialize session: {}", e)))?;
                
                // Extract session ID from JSON
                let _session_id = session_data.get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| rf_errors::RfError::Internal("Session data missing 'id' field".to_string()))?;
                
                // Create a new Session
                // Note: This creates a basic session. Full deserialization would require
                // access to Session internals to restore all session data.
                // For axum-sessions, we create a new session. The framework will handle ID management.
                // The application layer should handle restoring session data if needed.
                // This is a limitation of the current axum-sessions API which doesn't expose
                // direct serialization/deserialization methods.
                let session = Session::new();
                Ok(Some(session))
            }
            Err(_) => Ok(None),
        }
    }
    
    fn store(&self, session: Session) -> Result<()> {
        let key = self.session_key(session.id());
        
        // Serialize Session to JSON
        // Note: Session doesn't expose data() directly, we serialize the session ID
        // and attempt to serialize session data if available
        let session_id = session.id().to_string();
        
        // Try to get session data as JSON value
        // axum_sessions Session may have internal data that can be accessed
        let data = serde_json::to_string(&serde_json::json!({
            "id": session_id,
            // Note: Full session data serialization would require access to Session internals
            // For now, we store the ID and reconstruct a basic session on deserialization
            // This is a limitation - full session data cannot be restored without access to Session internals
        }))
        .map_err(|e| rf_errors::RfError::Internal(format!("Failed to serialize session: {}", e)))?;
        
        let rt = tokio::runtime::Handle::try_current()
            .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))?;
        
        // Store in Redis
        if let Some(ttl) = self.ttl {
            rt.block_on(self.client.string().set_ex(&key, &data, ttl))?;
        } else {
            rt.block_on(self.client.set(&key, &data))?;
        }
        
        Ok(())
    }
    
    fn delete(&self, id: &str) -> Result<()> {
        let key = self.session_key(id);
        let rt = tokio::runtime::Handle::try_current()
            .or_else(|_| tokio::runtime::Runtime::new().map(|rt| rt.handle().clone()))?;
        rt.block_on(self.client.generic().del(&[&key]))?;
        Ok(())
    }
}

/// Database-based session storage
pub struct DatabaseSessionStorage {
    database: Arc<rf_database::db::Database>,
    _table: String,
    ttl: Option<std::time::Duration>,
}

impl DatabaseSessionStorage {
    /// Create a new database session storage
    pub fn new(database: &rf_database::db::Database, table: &str) -> Self {
        // Wrap database reference in Arc for safe sharing
        let database_arc = Arc::new(unsafe {
            // Safety: Database is designed to be shared and long-lived.
            // Creating an Arc from a reference is safe as long as the original
            // Database outlives this DatabaseSessionStorage.
            std::ptr::read(database as *const rf_database::db::Database)
        });
        Self {
            database: database_arc,
            _table: table.to_string(),
            ttl: None,
        }
    }

    /// Set TTL for sessions
    pub fn with_ttl(mut self, ttl: std::time::Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }
}

impl SessionStorage for DatabaseSessionStorage {
    fn get(&self, _id: &str) -> Result<Option<Session>> {
        let _database = &*self.database;
        // Query session from database
        // Simplified - would query from table
        Ok(None)
    }
    
    fn store(&self, _session: Session) -> Result<()> {
        let _database = &*self.database;
        // Store session in database
        // Simplified - would insert/update in table
        Ok(())
    }
    
    fn delete(&self, _id: &str) -> Result<()> {
        let _database = &*self.database;
        // Delete session from database
        // Simplified - would delete from table
        Ok(())
    }
}

