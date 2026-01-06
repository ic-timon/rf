//! # replication
//!
//! replication 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Database replication (master-slave read-write split)

use super::database::Database;
use rf_errors::Result;
use std::sync::Arc;

/// Database replication manager for read-write splitting
pub struct ReplicationManager {
    master: Arc<Database>,
    slaves: Vec<Arc<Database>>,
    current_slave_index: Arc<std::sync::atomic::AtomicUsize>,
}

impl ReplicationManager {
    /// Create a new replication manager
    pub fn new(master: Database) -> Self {
        Self {
            master: Arc::new(master),
            slaves: Vec::new(),
            current_slave_index: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    /// Add a slave database
    pub fn add_slave(mut self, slave: Database) -> Self {
        self.slaves.push(Arc::new(slave));
        self
    }

    /// Get the master database (for write operations)
    pub fn master(&self) -> &Arc<Database> {
        &self.master
    }

    /// Get a slave database (for read operations) using round-robin
    pub fn slave(&self) -> Option<&Arc<Database>> {
        if self.slaves.is_empty() {
            return None;
        }
        let index = self.current_slave_index.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.slaves.get(index % self.slaves.len())
    }

    /// Get all slaves
    pub fn slaves(&self) -> &[Arc<Database>] {
        &self.slaves
    }

    /// Get slave count
    pub fn slave_count(&self) -> usize {
        self.slaves.len()
    }

    /// Check if replication is enabled (has slaves)
    pub fn is_enabled(&self) -> bool {
        !self.slaves.is_empty()
    }

    /// Get a database for read operations (slave if available, master otherwise)
    pub fn read_db(&self) -> &Arc<Database> {
        self.slave().unwrap_or(&self.master)
    }

    /// Get a database for write operations (always master)
    pub fn write_db(&self) -> &Arc<Database> {
        &self.master
    }
}

/// Load balancing strategy for slave selection
pub enum LoadBalanceStrategy {
    RoundRobin,
    Random,
    LeastConnections, // Would require connection pool monitoring
}

impl ReplicationManager {
    /// Get a slave using a specific strategy
    pub fn slave_with_strategy(&self, strategy: LoadBalanceStrategy) -> Option<&Arc<Database>> {
        if self.slaves.is_empty() {
            return None;
        }

        match strategy {
            LoadBalanceStrategy::RoundRobin => self.slave(),
            LoadBalanceStrategy::Random => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let index = rng.gen_range(0..self.slaves.len());
                self.slaves.get(index)
            }
            LoadBalanceStrategy::LeastConnections => {
                // For now, use round-robin as fallback
                // Full implementation would track connection counts
                self.slave()
            }
        }
    }
}

