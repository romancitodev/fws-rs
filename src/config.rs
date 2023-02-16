use serde::{Deserialize, Serialize};
use std::convert::From;
use std::error::Error;
use std::{fs::File, path::PathBuf};

#[derive(Debug, Clone)]
pub enum CheckMode {
    Recursive,
    NonRecursive,
}

#[derive(Serialize, Deserialize, Clone)]
struct JsonFile {
    watch: PathBuf,
    exec: String,
    recursive: bool,
    on_events_only: bool
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
        write!(f, "Error leyendo el archivo de configuración: {}", self.0)
    }
}

impl Error for ConfigError {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    path: PathBuf,
    exec: String,
    recursive: CheckMode,
    only_on_events: bool
}

#[allow(dead_code)]
impl Config {
    pub fn new(path: PathBuf, exec: String, recursive: bool, only_on_events: bool) -> Self {
        let recursive = match recursive {
            true => CheckMode::Recursive,
            false => CheckMode::NonRecursive,
        };
        Config {
            path,
            exec,
            recursive,
            only_on_events
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn exec(&self) -> &String {
        &self.exec
    }

    pub fn is_recursive(&self) -> bool {
        match &self.recursive {
            CheckMode::Recursive => true,
            CheckMode::NonRecursive => false,
        }
    }

    pub fn reload_on_events(&self) -> bool {
        self.only_on_events
    }

    pub fn load_from_args(path: PathBuf, exec: String, recursive: bool, only_on_events: bool) -> Self {
        Config::new(path, exec, recursive, only_on_events)
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let config: JsonFile = serde_json::from_reader(file)?;
        Ok(Config::new(
            PathBuf::from(&config.watch),
            config.exec,
            config.recursive,
            config.on_events_only
        ))
    }
}
