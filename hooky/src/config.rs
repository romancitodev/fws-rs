use crate::watcher::Interval;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct CommandConfig {
    supress_output: OutputMode,
    shell_mode: ShellMode,
}

#[allow(dead_code)]
impl CommandConfig {
    pub fn new(supress: OutputMode, shell: ShellMode) -> Self {
        Self {
            supress_output: supress,
            shell_mode: shell,
        }
    }

    pub fn output_mode(&self) -> OutputMode {
        self.supress_output
    }

    pub fn shell_mode(&self) -> ShellMode {
        self.shell_mode
    }
}

impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            supress_output: OutputMode::Allow,
            shell_mode: ShellMode::Cmd,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize)]
pub enum ShellMode {
    Shell,
    Cmd,
    Powershell,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Deserialize)]
pub enum OutputMode {
    Supress,
    Allow,
}

#[derive(Deserialize)]
pub struct SupervisorConfig {
    pub watcher_config: WatcherConfig,
    pub commands: Vec<String>,
}

#[derive(Deserialize)]
pub struct WatcherConfig {
    pub path: PathBuf,
    pub recursive: bool,
    pub patterns: Vec<String>,
    pub interval: Interval,
}

#[derive(Deserialize, Serialize)]
struct JsonFile {
    watch: PathBuf,
    exec: String,
    recursive: Option<bool>,
    attempts: Option<usize>,
    patterns: Option<Vec<String>>,
}

impl WatcherConfig {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn is_recursive(&self) -> bool {
        self.recursive
    }

    pub fn patterns(&self) -> Vec<String> {
        self.patterns.to_owned()
    }
}
