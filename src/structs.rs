use std::convert::From;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::{path::PathBuf, fs::File};

#[derive(Debug, Clone)]
pub enum CheckMode {
    Recursive,
    NonRecursive
}

#[derive(Serialize, Deserialize, Clone)]
struct JsonFile {
    watch: PathBuf,
    exec: String,
    recursive: bool
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    path: PathBuf,
    exec: String,
    recursive: CheckMode
}

#[allow(dead_code)]
impl Config {
    pub fn new(path: PathBuf, exec: String, recursive: bool) -> Self {
        let recursive = match recursive {
            true => CheckMode::Recursive,
            false => CheckMode::NonRecursive
        };
        Config { path, exec, recursive }
    }

    pub fn path(self: &Self) -> &PathBuf {
        &self.path
    }

    pub fn exec(self: &Self) -> &String {
        &self.exec
    }

    pub fn is_recursive(self: &Self) -> bool {
        match &self.recursive {
            CheckMode::Recursive => true,
            CheckMode::NonRecursive => false
        }
    }

    pub fn load_from_args(path: PathBuf, exec: String, recursive: bool) -> Self {
        Config::new(path, exec, recursive)
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let config: JsonFile = serde_json::from_reader(file)?;
        Ok(Config::new(PathBuf::from(&config.watch), config.exec, config.recursive))
    }
}
