//! # cmd
//!
//! cmd 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Command line parsing

use clap::{Command, Arg, ArgMatches};

/// Command line parser
pub struct Cmd {
    command: Command,
}

impl Cmd {
    /// Create a new command parser
    pub fn new(name: &'static str) -> Self {
        Self {
            command: Command::new(name),
        }
    }

    /// Add a subcommand
    pub fn subcommand(mut self, subcommand: Command) -> Self {
        self.command = self.command.subcommand(subcommand);
        self
    }

    /// Add an argument
    pub fn arg(mut self, arg: Arg) -> Self {
        self.command = self.command.arg(arg);
        self
    }

    /// Add a simple argument with name and help
    pub fn arg_simple(mut self, name: &'static str, help: &'static str) -> Self {
        self.command = self.command.arg(Arg::new(name).help(help));
        self
    }

    /// Add a required argument
    pub fn arg_required(mut self, name: &'static str, help: &'static str) -> Self {
        self.command = self.command.arg(Arg::new(name).required(true).help(help));
        self
    }

    /// Add an argument with default value
    pub fn arg_default(mut self, name: &'static str, default: &'static str, help: &'static str) -> Self {
        self.command = self.command.arg(Arg::new(name).default_value(default).help(help));
        self
    }

    /// Add an argument that takes multiple values
    pub fn arg_multiple(mut self, name: &'static str, help: &'static str) -> Self {
        self.command = self.command.arg(Arg::new(name).num_args(1..).help(help));
        self
    }

    /// Set the version
    pub fn version(mut self, version: &'static str) -> Self {
        self.command = self.command.version(version);
        self
    }

    /// Set the author
    pub fn author(mut self, author: &'static str) -> Self {
        self.command = self.command.author(author);
        self
    }

    /// Set the about/description
    pub fn about(mut self, about: &'static str) -> Self {
        self.command = self.command.about(about);
        self
    }

    /// Enable environment variable binding for all arguments
    /// Note: clap 4.x doesn't have env_global, use arg().env() for individual args
    pub fn env_global(self) -> Self {
        // In clap 4.x, environment variables are handled per-argument
        // This is a no-op for now - users should use arg().env() for each argument
        self
    }

    /// Parse command line arguments
    pub fn parse(self) -> ArgMatches {
        self.command.get_matches()
    }

    /// Parse command line arguments (try version that doesn't exit on error)
    pub fn try_parse(self) -> Result<ArgMatches, clap::Error> {
        self.command.try_get_matches()
    }

    /// Get an argument value (requires parsing first)
    pub fn get_arg(matches: &ArgMatches, name: &str) -> Option<String> {
        matches.get_one::<String>(name).cloned()
    }

    /// Get multiple argument values
    pub fn get_args(matches: &ArgMatches, name: &str) -> Vec<String> {
        matches.get_many::<String>(name)
            .map(|vals| vals.cloned().collect())
            .unwrap_or_default()
    }

    /// Check if an argument exists (requires parsing first)
    pub fn has_arg(matches: &ArgMatches, name: &str) -> bool {
        matches.contains_id(name)
    }

    /// Get a subcommand matches
    pub fn get_subcommand<'a>(matches: &'a ArgMatches, name: &str) -> Option<&'a ArgMatches> {
        matches.subcommand_matches(name)
    }

    /// Check if a subcommand was used
    pub fn has_subcommand(matches: &ArgMatches, name: &str) -> bool {
        matches.subcommand_matches(name).is_some()
    }

    /// Print help information
    pub fn print_help(&mut self) {
        let _ = self.command.print_help();
    }

    /// Print long help information
    pub fn print_long_help(&mut self) {
        let _ = self.command.print_long_help();
    }
}

