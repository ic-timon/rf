//! # proc
//!
//! proc 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Process management

use sysinfo::{Pid, System};
use tokio::process::Command;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::Instant;

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub status: String,
}

/// Process monitor
pub struct ProcessMonitor {
    system: Arc<RwLock<System>>,
}

impl ProcessMonitor {
    /// Create a new process monitor
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self {
            system: Arc::new(RwLock::new(system)),
        }
    }

    /// Refresh process information
    pub async fn refresh(&self) {
        let mut system = self.system.write().await;
        system.refresh_all();
    }

    /// Get process information by PID
    pub async fn get_process(&self, pid: u32) -> Option<ProcessInfo> {
        let system = self.system.read().await;
        system.process(Pid::from(pid as usize)).map(|process| ProcessInfo {
                pid,
                name: process.name().to_string_lossy().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                status: format!("{:?}", process.status()),
            })
    }

    /// Get all processes
    pub async fn get_all_processes(&self) -> Vec<ProcessInfo> {
        let system = self.system.read().await;
        system.processes()
            .iter()
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                status: format!("{:?}", process.status()),
            })
            .collect()
    }

    /// Find processes by name
    pub async fn find_by_name(&self, name: &str) -> Vec<ProcessInfo> {
        let system = self.system.read().await;
        system.processes()
            .iter()
            .filter(|(_, process)| process.name().to_string_lossy().contains(name))
            .map(|(pid, process)| ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string_lossy().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                status: format!("{:?}", process.status()),
            })
            .collect()
    }

    /// Check if process is running
    pub async fn is_running(&self, pid: u32) -> bool {
        let system = self.system.read().await;
        system.process(Pid::from(pid as usize)).is_some()
    }
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Process pool for managing multiple processes
pub struct ProcessPool {
    processes: Arc<RwLock<HashMap<u32, ProcessHandle>>>,
    monitor: ProcessMonitor,
}

struct ProcessHandle {
    _pid: u32,
    _started_at: Instant,
    _last_health_check: Instant,
}

impl ProcessPool {
    /// Create a new process pool
    pub fn new() -> Self {
        Self {
            processes: Arc::new(RwLock::new(HashMap::new())),
            monitor: ProcessMonitor::new(),
        }
    }

    /// Add a process to the pool
    pub async fn add(&self, pid: u32) {
        let mut processes = self.processes.write().await;
        processes.insert(pid, ProcessHandle {
            _pid: pid,
            _started_at: Instant::now(),
            _last_health_check: Instant::now(),
        });
    }

    /// Remove a process from the pool
    pub async fn remove(&self, pid: u32) {
        let mut processes = self.processes.write().await;
        processes.remove(&pid);
    }

    /// Check health of all processes in the pool
    pub async fn health_check(&self) -> HashMap<u32, bool> {
        self.monitor.refresh().await;
        let mut results = HashMap::new();
        let processes = self.processes.read().await;
        
        for (pid, _handle) in processes.iter() {
            let is_running = self.monitor.is_running(*pid).await;
            results.insert(*pid, is_running);
        }
        
        results
    }

    /// Get all process IDs in the pool
    pub async fn get_pids(&self) -> Vec<u32> {
        let processes = self.processes.read().await;
        processes.keys().copied().collect()
    }
}

impl Default for ProcessPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Execute a command
pub async fn exec(cmd: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .await?;
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Execute a command and return the process handle
pub async fn exec_async(cmd: &str, args: &[&str]) -> Result<tokio::process::Child, Box<dyn std::error::Error>> {
    let child = Command::new(cmd)
        .args(args)
        .spawn()?;
    
    Ok(child)
}

/// Kill a process by PID
pub async fn kill(pid: u32) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        use std::process::Command as StdCommand;
        StdCommand::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .output()?;
    }
    
    #[cfg(windows)]
    {
        use std::process::Command as StdCommand;
        StdCommand::new("taskkill")
            .arg("/F")
            .arg("/PID")
            .arg(pid.to_string())
            .output()?;
    }
    
    Ok(())
}

