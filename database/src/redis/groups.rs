//! # groups
//!
//! groups 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Redis operation groups

use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use rf_errors::{Result, RfError};
use std::sync::Arc;
use tokio::sync::Mutex;

/// String operations group
pub struct StringGroup {
    pub(crate) connection: Arc<Mutex<MultiplexedConnection>>,
}

impl StringGroup {
    /// Set a key-value pair
    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.connection.lock().await;
        conn.set::<_, _, ()>(key, value).await
            .map_err(|e| RfError::Database(format!("Redis SET failed: {}", e)))?;
        Ok(())
    }

    /// Set with expiration (seconds)
    pub async fn set_ex(&self, key: &str, value: &str, seconds: u64) -> Result<()> {
        let mut conn = self.connection.lock().await;
        conn.set_ex::<_, _, ()>(key, value, seconds).await
            .map_err(|e| RfError::Database(format!("Redis SETEX failed: {}", e)))?;
        Ok(())
    }

    /// Set if not exists
    pub async fn set_nx(&self, key: &str, value: &str) -> Result<bool> {
        let mut conn = self.connection.lock().await;
        conn.set_nx::<_, _, bool>(key, value).await
            .map_err(|e| RfError::Database(format!("Redis SETNX failed: {}", e)))
    }

    /// Get a value by key
    pub async fn get(&self, key: &str) -> Result<String> {
        let mut conn = self.connection.lock().await;
        conn.get::<_, String>(key).await
            .map_err(|e| RfError::Database(format!("Redis GET failed: {}", e)))
    }

    /// Get and delete
    pub async fn get_del(&self, key: &str) -> Result<String> {
        let mut conn = self.connection.lock().await;
        redis::cmd("GETDEL").arg(key).query_async::<String>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis GETDEL failed: {}", e)))
    }

    /// Get and set
    pub async fn get_set(&self, key: &str, value: &str) -> Result<String> {
        let mut conn = self.connection.lock().await;
        conn.getset::<_, _, String>(key, value).await
            .map_err(|e| RfError::Database(format!("Redis GETSET failed: {}", e)))
    }

    /// Get string length
    pub async fn strlen(&self, key: &str) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        conn.strlen::<_, usize>(key).await
            .map_err(|e| RfError::Database(format!("Redis STRLEN failed: {}", e)))
    }

    /// Append to string
    pub async fn append(&self, key: &str, value: &str) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        conn.append::<_, _, usize>(key, value).await
            .map_err(|e| RfError::Database(format!("Redis APPEND failed: {}", e)))
    }

    /// Increment
    pub async fn incr(&self, key: &str) -> Result<i64> {
        let mut conn = self.connection.lock().await;
        conn.incr::<_, _, i64>(key, 1).await
            .map_err(|e| RfError::Database(format!("Redis INCR failed: {}", e)))
    }

    /// Increment by
    pub async fn incr_by(&self, key: &str, increment: i64) -> Result<i64> {
        let mut conn = self.connection.lock().await;
        conn.incr::<_, _, i64>(key, increment).await
            .map_err(|e| RfError::Database(format!("Redis INCRBY failed: {}", e)))
    }

    /// Increment by float
    pub async fn incr_by_float(&self, key: &str, increment: f64) -> Result<f64> {
        let mut conn = self.connection.lock().await;
        redis::cmd("INCRBYFLOAT").arg(key).arg(increment).query_async::<f64>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis INCRBYFLOAT failed: {}", e)))
    }

    /// Decrement
    pub async fn decr(&self, key: &str) -> Result<i64> {
        let mut conn = self.connection.lock().await;
        conn.decr::<_, _, i64>(key, 1).await
            .map_err(|e| RfError::Database(format!("Redis DECR failed: {}", e)))
    }

    /// Decrement by
    pub async fn decr_by(&self, key: &str, decrement: i64) -> Result<i64> {
        let mut conn = self.connection.lock().await;
        conn.decr::<_, _, i64>(key, decrement).await
            .map_err(|e| RfError::Database(format!("Redis DECRBY failed: {}", e)))
    }

    /// Multiple set
    pub async fn mset(&self, pairs: &[(&str, &str)]) -> Result<()> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("MSET");
        for (key, value) in pairs {
            cmd.arg(key).arg(value);
        }
        cmd.query_async::<()>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis MSET failed: {}", e)))
    }

    /// Multiple get
    pub async fn mget(&self, keys: &[&str]) -> Result<Vec<String>> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("MGET");
        for key in keys {
            cmd.arg(key);
        }
        cmd.query_async::<Vec<String>>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis MGET failed: {}", e)))
    }
}

/// Hash operations group
pub struct HashGroup {
    pub(crate) connection: Arc<Mutex<MultiplexedConnection>>,
}

impl HashGroup {
    /// Set hash field
    pub async fn hset(&self, key: &str, field: &str, value: &str) -> Result<()> {
        let mut conn = self.connection.lock().await;
        conn.hset::<_, _, _, ()>(key, field, value).await
            .map_err(|e| RfError::Database(format!("Redis HSET failed: {}", e)))?;
        Ok(())
    }

    /// Get hash field
    pub async fn hget(&self, key: &str, field: &str) -> Result<String> {
        let mut conn = self.connection.lock().await;
        conn.hget::<_, _, String>(key, field).await
            .map_err(|e| RfError::Database(format!("Redis HGET failed: {}", e)))
    }

    /// Delete hash fields
    pub async fn hdel(&self, key: &str, fields: &[&str]) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("HDEL");
        cmd.arg(key);
        for field in fields {
            cmd.arg(field);
        }
        cmd.query_async::<usize>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis HDEL failed: {}", e)))
    }

    /// Check if hash field exists
    pub async fn hexists(&self, key: &str, field: &str) -> Result<bool> {
        let mut conn = self.connection.lock().await;
        conn.hexists::<_, _, bool>(key, field).await
            .map_err(|e| RfError::Database(format!("Redis HEXISTS failed: {}", e)))
    }

    /// Get hash length
    pub async fn hlen(&self, key: &str) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        conn.hlen::<_, usize>(key).await
            .map_err(|e| RfError::Database(format!("Redis HLEN failed: {}", e)))
    }

    /// Get all hash fields and values
    pub async fn hgetall(&self, key: &str) -> Result<std::collections::HashMap<String, String>> {
        let mut conn = self.connection.lock().await;
        conn.hgetall::<_, std::collections::HashMap<String, String>>(key).await
            .map_err(|e| RfError::Database(format!("Redis HGETALL failed: {}", e)))
    }

    /// Get hash keys
    pub async fn hkeys(&self, key: &str) -> Result<Vec<String>> {
        let mut conn = self.connection.lock().await;
        conn.hkeys::<_, Vec<String>>(key).await
            .map_err(|e| RfError::Database(format!("Redis HKEYS failed: {}", e)))
    }

    /// Get hash values
    pub async fn hvals(&self, key: &str) -> Result<Vec<String>> {
        let mut conn = self.connection.lock().await;
        conn.hvals::<_, Vec<String>>(key).await
            .map_err(|e| RfError::Database(format!("Redis HVALS failed: {}", e)))
    }

    /// Increment hash field by integer
    pub async fn hincr_by(&self, key: &str, field: &str, increment: i64) -> Result<i64> {
        let mut conn = self.connection.lock().await;
        conn.hincr::<_, _, _, i64>(key, field, increment).await
            .map_err(|e| RfError::Database(format!("Redis HINCRBY failed: {}", e)))
    }

    /// Multiple set hash fields
    pub async fn hmset(&self, key: &str, pairs: &[(&str, &str)]) -> Result<()> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("HMSET");
        cmd.arg(key);
        for (field, value) in pairs {
            cmd.arg(field).arg(value);
        }
        cmd.query_async::<()>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis HMSET failed: {}", e)))
    }

    /// Multiple get hash fields
    pub async fn hmget(&self, key: &str, fields: &[&str]) -> Result<Vec<String>> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("HMGET");
        cmd.arg(key);
        for field in fields {
            cmd.arg(field);
        }
        cmd.query_async::<Vec<String>>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis HMGET failed: {}", e)))
    }
}

/// List operations group
pub struct ListGroup {
    pub(crate) connection: Arc<Mutex<MultiplexedConnection>>,
}

impl ListGroup {
    /// Left push
    pub async fn lpush(&self, key: &str, values: &[&str]) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("LPUSH");
        cmd.arg(key);
        for value in values {
            cmd.arg(value);
        }
        cmd.query_async::<usize>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis LPUSH failed: {}", e)))
    }

    /// Right push
    pub async fn rpush(&self, key: &str, values: &[&str]) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("RPUSH");
        cmd.arg(key);
        for value in values {
            cmd.arg(value);
        }
        cmd.query_async::<usize>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis RPUSH failed: {}", e)))
    }

    /// Left pop
    pub async fn lpop(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.connection.lock().await;
        conn.lpop::<_, Option<String>>(key, None).await
            .map_err(|e| RfError::Database(format!("Redis LPOP failed: {}", e)))
    }

    /// Right pop
    pub async fn rpop(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.connection.lock().await;
        conn.rpop::<_, Option<String>>(key, None).await
            .map_err(|e| RfError::Database(format!("Redis RPOP failed: {}", e)))
    }

    /// Get list length
    pub async fn llen(&self, key: &str) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        conn.llen::<_, usize>(key).await
            .map_err(|e| RfError::Database(format!("Redis LLEN failed: {}", e)))
    }

    /// Get list element by index
    pub async fn lindex(&self, key: &str, index: i64) -> Result<Option<String>> {
        let mut conn = self.connection.lock().await;
        conn.lindex::<_, Option<String>>(key, index as isize).await
            .map_err(|e| RfError::Database(format!("Redis LINDEX failed: {}", e)))
    }

    /// Get list range
    pub async fn lrange(&self, key: &str, start: i64, stop: i64) -> Result<Vec<String>> {
        let mut conn = self.connection.lock().await;
        conn.lrange::<_, Vec<String>>(key, start as isize, stop as isize).await
            .map_err(|e| RfError::Database(format!("Redis LRANGE failed: {}", e)))
    }

    /// Trim list
    pub async fn ltrim(&self, key: &str, start: i64, stop: i64) -> Result<()> {
        let mut conn = self.connection.lock().await;
        conn.ltrim::<_, ()>(key, start as isize, stop as isize).await
            .map_err(|e| RfError::Database(format!("Redis LTRIM failed: {}", e)))?;
        Ok(())
    }
}

/// Set operations group
pub struct SetGroup {
    pub(crate) connection: Arc<Mutex<MultiplexedConnection>>,
}

impl SetGroup {
    /// Add members to set
    pub async fn sadd(&self, key: &str, members: &[&str]) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("SADD");
        cmd.arg(key);
        for member in members {
            cmd.arg(member);
        }
        cmd.query_async::<usize>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis SADD failed: {}", e)))
    }

    /// Check if member exists in set
    pub async fn sismember(&self, key: &str, member: &str) -> Result<bool> {
        let mut conn = self.connection.lock().await;
        conn.sismember::<_, _, bool>(key, member).await
            .map_err(|e| RfError::Database(format!("Redis SISMEMBER failed: {}", e)))
    }

    /// Get set size
    pub async fn scard(&self, key: &str) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        conn.scard::<_, usize>(key).await
            .map_err(|e| RfError::Database(format!("Redis SCARD failed: {}", e)))
    }

    /// Get all set members
    pub async fn smembers(&self, key: &str) -> Result<Vec<String>> {
        let mut conn = self.connection.lock().await;
        conn.smembers::<_, Vec<String>>(key).await
            .map_err(|e| RfError::Database(format!("Redis SMEMBERS failed: {}", e)))
    }

    /// Remove members from set
    pub async fn srem(&self, key: &str, members: &[&str]) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("SREM");
        cmd.arg(key);
        for member in members {
            cmd.arg(member);
        }
        cmd.query_async::<usize>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis SREM failed: {}", e)))
    }
}

/// SortedSet operations group
pub struct SortedSetGroup {
    pub(crate) connection: Arc<Mutex<MultiplexedConnection>>,
}

impl SortedSetGroup {
    /// Add member with score
    pub async fn zadd(&self, key: &str, score: f64, member: &str) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        conn.zadd::<_, _, _, usize>(key, member, score).await
            .map_err(|e| RfError::Database(format!("Redis ZADD failed: {}", e)))
    }

    /// Get sorted set size
    pub async fn zcard(&self, key: &str) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        conn.zcard::<_, usize>(key).await
            .map_err(|e| RfError::Database(format!("Redis ZCARD failed: {}", e)))
    }

    /// Get member score
    pub async fn zscore(&self, key: &str, member: &str) -> Result<Option<f64>> {
        let mut conn = self.connection.lock().await;
        conn.zscore::<_, _, Option<f64>>(key, member).await
            .map_err(|e| RfError::Database(format!("Redis ZSCORE failed: {}", e)))
    }

    /// Get range by rank
    pub async fn zrange(&self, key: &str, start: i64, stop: i64) -> Result<Vec<String>> {
        let mut conn = self.connection.lock().await;
        conn.zrange::<_, Vec<String>>(key, start as isize, stop as isize).await
            .map_err(|e| RfError::Database(format!("Redis ZRANGE failed: {}", e)))
    }

    /// Remove members
    pub async fn zrem(&self, key: &str, members: &[&str]) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("ZREM");
        cmd.arg(key);
        for member in members {
            cmd.arg(member);
        }
        cmd.query_async::<usize>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis ZREM failed: {}", e)))
    }
}

/// Generic operations group
pub struct GenericGroup {
    pub(crate) connection: Arc<Mutex<MultiplexedConnection>>,
}

impl GenericGroup {
    /// Check if key exists
    pub async fn exists(&self, keys: &[&str]) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("EXISTS");
        for key in keys {
            cmd.arg(key);
        }
        cmd.query_async::<usize>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis EXISTS failed: {}", e)))
    }

    /// Delete keys
    pub async fn del(&self, keys: &[&str]) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("DEL");
        for key in keys {
            cmd.arg(key);
        }
        cmd.query_async::<usize>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis DEL failed: {}", e)))
    }

    /// Get key type
    pub async fn r#type(&self, key: &str) -> Result<String> {
        let mut conn = self.connection.lock().await;
        redis::cmd("TYPE").arg(key).query_async::<String>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis TYPE failed: {}", e)))
    }

    /// Set expiration (seconds)
    pub async fn expire(&self, key: &str, seconds: usize) -> Result<bool> {
        let mut conn = self.connection.lock().await;
        conn.expire::<_, bool>(key, seconds as i64).await
            .map_err(|e| RfError::Database(format!("Redis EXPIRE failed: {}", e)))
    }

    /// Get TTL
    pub async fn ttl(&self, key: &str) -> Result<i64> {
        let mut conn = self.connection.lock().await;
        conn.ttl::<_, i64>(key).await
            .map_err(|e| RfError::Database(format!("Redis TTL failed: {}", e)))
    }

    /// Get keys by pattern
    pub async fn keys(&self, pattern: &str) -> Result<Vec<String>> {
        let mut conn = self.connection.lock().await;
        conn.keys::<_, Vec<String>>(pattern).await
            .map_err(|e| RfError::Database(format!("Redis KEYS failed: {}", e)))
    }

    /// Rename key
    pub async fn rename(&self, key: &str, new_key: &str) -> Result<()> {
        let mut conn = self.connection.lock().await;
        conn.rename::<_, _, ()>(key, new_key).await
            .map_err(|e| RfError::Database(format!("Redis RENAME failed: {}", e)))?;
        Ok(())
    }
}

/// PubSub operations group
pub struct PubSubGroup {
    pub(crate) connection: Arc<Mutex<MultiplexedConnection>>,
}

impl PubSubGroup {
    /// Publish message to channel
    pub async fn publish(&self, channel: &str, message: &str) -> Result<usize> {
        let mut conn = self.connection.lock().await;
        conn.publish::<_, _, usize>(channel, message).await
            .map_err(|e| RfError::Database(format!("Redis PUBLISH failed: {}", e)))
    }
}

/// Script operations group
pub struct ScriptGroup {
    pub(crate) connection: Arc<Mutex<MultiplexedConnection>>,
}

impl ScriptGroup {
    /// Execute Lua script
    pub async fn eval(&self, script: &str, keys: &[&str], args: &[&str]) -> Result<redis::Value> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("EVAL");
        cmd.arg(script).arg(keys.len());
        for key in keys {
            cmd.arg(key);
        }
        for arg in args {
            cmd.arg(arg);
        }
        cmd.query_async::<redis::Value>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis EVAL failed: {}", e)))
    }

    /// Execute Lua script by SHA
    pub async fn evalsha(&self, sha: &str, keys: &[&str], args: &[&str]) -> Result<redis::Value> {
        let mut conn = self.connection.lock().await;
        let mut cmd = redis::cmd("EVALSHA");
        cmd.arg(sha).arg(keys.len());
        for key in keys {
            cmd.arg(key);
        }
        for arg in args {
            cmd.arg(arg);
        }
        cmd.query_async::<redis::Value>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis EVALSHA failed: {}", e)))
    }

    /// Load script
    pub async fn script_load(&self, script: &str) -> Result<String> {
        let mut conn = self.connection.lock().await;
        redis::cmd("SCRIPT").arg("LOAD").arg(script).query_async::<String>(&mut *conn).await
            .map_err(|e| RfError::Database(format!("Redis SCRIPT LOAD failed: {}", e)))
    }
}

