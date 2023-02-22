use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use std::convert::From;
use std::env;
use std::error::Error;
use std::{fs::File, path::PathBuf};

#[derive(Serialize, Deserialize, Clone)]

/// watch: path to observe (required)
///
/// exec: command to execute (required)
///
/// recursive: if the path to observer will be recursive (optional) default: False
///
/// on_events_only: if the command will be execute only on events (optional) default: False
///
/// attemps: attempts to restart the command if fails (optional) default: 3
struct JsonFile {
    watch: PathBuf,
    exec: String,
    recursive: Option<bool>,
    on_events_only: Option<bool>,
    attempts: Option<usize>,
    patterns: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct ConfigError(serde_json::Error);

impl From<serde_json::Error> for ConfigError {
    fn from(error: serde_json::Error) -> Self {
        ConfigError(error)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error reading configuration file: {}", self.0)
    }
}

impl Error for ConfigError {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    path: PathBuf,
    exec: String,
    recursive: bool,
    only_on_events: bool,
    patterns: Vec<String>,
}

#[allow(dead_code)]
impl Config {
    pub fn new(
        path: PathBuf,
        exec: String,
        recursive: bool,
        only_on_events: bool,
        patterns: Vec<String>,
    ) -> Self {
        Config {
            path,
            exec,
            recursive,
            only_on_events,
            patterns,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn exec(&self) -> &String {
        &self.exec
    }

    pub fn is_recursive(&self) -> bool {
        self.recursive
    }

    pub fn only_events(&self) -> bool {
        self.only_on_events
    }

    pub fn patterns(&self) -> Vec<String> {
        self.patterns.to_owned()
    }

    pub fn load_from_args(
        path: PathBuf,
        exec: String,
        recursive: bool,
        only_on_events: bool,
        patterns: Vec<String>,
    ) -> Self {
        Config::new(path, exec, recursive, only_on_events, patterns)
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let config: JsonFile = serde_json::from_reader(file)?;
        Ok(Config::new(
            PathBuf::from(&config.watch),
            config.exec,
            config.recursive.unwrap_or(false),
            config.on_events_only.unwrap_or(false),
            config.patterns.unwrap_or(Vec::<String>::new()),
        ))
    }
}

pub fn get_config_file() -> Option<PathBuf> {
    let root = env::current_dir().unwrap();
    match WalkDir::new(root).into_iter().find(|x| x.as_ref().unwrap().file_name().to_str() == Some(&"observer.json")) {
        Some(file) => Some(file.unwrap().into_path()),
        None => None
    }
}