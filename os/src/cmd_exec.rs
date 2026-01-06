//! # cmd_exec
//!
//! cmd_exec 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Command execution utilities

use tokio::process::Command;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Command execution result
#[derive(Debug)]
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub success: bool,
}

/// Command builder with enhanced features
pub struct CommandBuilder {
    command: Command,
    env_vars: HashMap<String, String>,
    timeout: Option<Duration>,
    working_dir: Option<PathBuf>,
    background: bool,
}

impl CommandBuilder {
    /// Create a new command builder
    pub fn new(cmd: &str) -> Self {
        let mut command = Command::new(cmd);
        command.kill_on_drop(true);
        
        Self {
            command,
            env_vars: HashMap::new(),
            timeout: None,
            working_dir: None,
            background: false,
        }
    }

    /// Add an argument
    pub fn arg(mut self, arg: impl AsRef<str>) -> Self {
        self.command.arg(arg.as_ref());
        self
    }

    /// Add multiple arguments
    pub fn args(mut self, args: &[impl AsRef<str>]) -> Self {
        self.command.args(args.iter().map(|a| a.as_ref()));
        self
    }

    /// Set environment variable
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let value = value.into();
        self.env_vars.insert(key.clone(), value.clone());
        self.command.env(&key, &value);
        self
    }

    /// Clear environment variables
    pub fn env_clear(mut self) -> Self {
        self.command.env_clear();
        self.env_vars.clear();
        self
    }

    /// Set working directory
    pub fn current_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        let dir = dir.into();
        self.working_dir = Some(dir.clone());
        self.command.current_dir(dir);
        self
    }

    /// Set timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Run in background
    pub fn background(mut self) -> Self {
        self.background = true;
        self
    }

    /// Execute the command
    pub async fn execute(mut self) -> Result<CommandResult, Box<dyn std::error::Error>> {
        if self.background {
            let _child = self.command.spawn()?;
            // Don't wait for background process
            return Ok(CommandResult {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: None,
                success: true,
            });
        }

        let output = if let Some(timeout) = self.timeout {
            tokio::time::timeout(timeout, self.command.output()).await?
        } else {
            self.command.output().await
        }?;

        let exit_code = output.status.code();
        let success = output.status.success();

        Ok(CommandResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code,
            success,
        })
    }

    /// Execute and get real-time output
    pub async fn execute_stream(
        mut self,
    ) -> Result<tokio::process::Child, Box<dyn std::error::Error>> {
        let child = self.command
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        Ok(child)
    }
}

/// Command history manager
pub struct CommandHistory {
    history: Arc<RwLock<Vec<String>>>,
    max_size: usize,
}

impl CommandHistory {
    /// Create a new command history manager
    pub fn new(max_size: usize) -> Self {
        Self {
            history: Arc::new(RwLock::new(Vec::new())),
            max_size,
        }
    }

    /// Add a command to history
    pub async fn add(&self, command: String) {
        let mut history = self.history.write().await;
        history.push(command);
        if history.len() > self.max_size {
            history.remove(0);
        }
    }

    /// Get command history
    pub async fn get_history(&self, limit: Option<usize>) -> Vec<String> {
        let history = self.history.read().await;
        let limit = limit.unwrap_or(history.len());
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Search history
    pub async fn search(&self, query: &str) -> Vec<String> {
        let history = self.history.read().await;
        history
            .iter()
            .rev()
            .filter(|cmd| cmd.contains(query))
            .take(10)
            .cloned()
            .collect()
    }

    /// Clear history
    pub async fn clear(&self) {
        let mut history = self.history.write().await;
        history.clear();
    }
}

impl Default for CommandHistory {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Execute command with pipe support (simplified)
pub async fn pipe_commands(
    commands: Vec<CommandBuilder>,
) -> Result<CommandResult, Box<dyn std::error::Error>> {
    // This is a simplified pipe implementation
    // Full implementation would properly chain stdout to stdin
    if commands.is_empty() {
        return Err("No commands to pipe".into());
    }

    // For now, execute commands sequentially
    let mut last_output = String::new();
    for (i, cmd_builder) in commands.into_iter().enumerate() {
        if i > 0 {
            // In full implementation, pipe last_output to stdin
            // For now, just execute sequentially
        }
        let result = cmd_builder.execute().await?;
        last_output = result.stdout;
    }

    Ok(CommandResult {
        stdout: last_output,
        stderr: String::new(),
        exit_code: Some(0),
        success: true,
    })
}

