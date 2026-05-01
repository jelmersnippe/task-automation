use std::{
    collections::HashMap,
    fmt,
    process::{Command, Stdio},
};

use crate::{
    interpreter::{builtin::BuiltinFn, datatype::Callable},
    modules::{git::create_git_module, shell::create_shell_module, tmux::create_tmux_module},
};

pub mod cmd;
pub mod git;
pub mod shell;
pub mod tmux;

#[derive(Debug, Clone)]
pub struct GitError {
    pub command: String,
    pub reason: String,
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Git command '{}' failed: {}", self.command, self.reason)
    }
}

pub trait GitRunner: Send + Sync {
    fn run(&self, args: &[&str], cwd: &str) -> Result<String, GitError>;
}

pub struct ProcessGitRunner;

impl GitRunner for ProcessGitRunner {
    fn run(&self, args: &[&str], cwd: &str) -> Result<String, GitError> {
        let output = Command::new("git")
            .current_dir(cwd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|error| GitError {
                reason: error.to_string(),
                command: format!("git {}", args.join(" ")),
            })?;

        if !output.status.success() {
            return Err(GitError {
                reason: String::from_utf8_lossy(&output.stderr).to_string(),
                command: format!("git {}", args.join(" ")),
            });
        }

        let stdout = String::from_utf8(output.stdout).map_err(|e| GitError {
            reason: e.to_string(),
            command: format!("git {}", args.join(" ")),
        })?;

        Ok(stdout)
    }
}

#[derive(Debug, Clone)]
pub struct TmuxError {
    pub command: String,
    pub reason: String,
}

impl fmt::Display for TmuxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tmux command '{}' failed: {}", self.command, self.reason)
    }
}

pub trait TmuxRunner: Send + Sync {
    fn run(&self, args: &[&str]) -> Result<String, TmuxError>;
}

pub struct ProcessTmuxRunner;

impl TmuxRunner for ProcessTmuxRunner {
    fn run(&self, args: &[&str]) -> Result<String, TmuxError> {
        let output = Command::new("tmux")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|error| TmuxError {
                reason: error.to_string(),
                command: format!("tmux {}", args.join(" ")),
            })?;

        if !output.status.success() {
            return Err(TmuxError {
                reason: String::from_utf8_lossy(&output.stderr).to_string(),
                command: format!("tmux {}", args.join(" ")),
            });
        }

        let stdout = String::from_utf8(output.stdout).map_err(|e| TmuxError {
            reason: e.to_string(),
            command: format!("tmux {}", args.join(" ")),
        })?;

        Ok(stdout)
    }
}

#[derive(Clone)]
pub struct Module {
    pub name: String,
    pub functions: HashMap<String, Callable>,
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Module").field("name", &self.name).finish()
    }
}

impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            functions: Default::default(),
        }
    }

    pub fn function(mut self, name: &str, function: BuiltinFn) -> Self {
        self.functions.insert(
            name.to_string(),
            Callable::new(Some(name.to_string()), function),
        );
        self
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Module '{}'", self.name)
    }
}

#[derive(Clone)]
pub struct ModuleRegistry {
    pub modules: Vec<Module>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Default::default(),
        }
    }

    pub fn register(&mut self, module: Module) {
        self.modules.push(module);
    }
}

pub fn git_module() -> Module {
    create_git_module()
}

pub fn shell_module() -> Module {
    create_shell_module()
}

pub fn tmux_module() -> Module {
    create_tmux_module()
}
