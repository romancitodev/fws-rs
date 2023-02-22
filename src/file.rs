use std::fs::Metadata;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use walkdir::DirEntry;
use std::collections::HashMap;
use std::time::Duration;
use walkdir::WalkDir;

use crate::config::Config;
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct File {
    name: String,
    path: PathBuf,
    data: FileData,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
struct FileData {
    last_accesed: SystemTime,
    last_modified: SystemTime,
}

#[allow(dead_code)]
impl File {
    pub fn new(file: &DirEntry) -> Self {
        let metadata = file.metadata().unwrap();
        File {
            name: file.file_name().to_str().unwrap().to_string(),
            path: file.path().to_path_buf(),
            data: FileData::new(metadata),
        }
    }

    pub fn name(&self) -> String {
        Arc::new(&self.name).to_string()
    }

    pub fn extension(&self) -> String {
        let binding = self.name.to_string();
        let extensions = binding
            .split('.')
            .collect::<Vec<_>>()
            .drain(1..)
            .collect::<Vec<_>>();
        format!(".{}", extensions.join("."))
    }

    pub fn ds_path(&self) -> String {
        Arc::new(&self.path)
            .to_path_buf()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn was_deleted(&self) -> bool {
        !self.path.exists()
    }

    pub fn last_modification(&self) -> SystemTime {
        self.data.last_modified
    }

    pub fn set_modification(&mut self, time: SystemTime) {
        self.data.last_modified = time
    }
}

impl FileData {
    pub fn new(metadata: Metadata) -> Self {
        FileData {
            last_accesed: metadata.accessed().unwrap(),
            last_modified: metadata.modified().unwrap(),
        }
    }
}

// events

#[derive(Debug)]
#[allow(dead_code)]
pub enum FileEvent {
    Created(File),
    Modified(File),
    Eliminated(File),
}

// Observer


#[allow(dead_code)]
#[derive(Debug)]
pub struct Observer {
    config: Config,
    files: HashMap<String, File>,
}

fn get_files(config: &Config) -> HashMap<String, File> {
    let files = match config.is_recursive() {
        true => WalkDir::new(config.path()).min_depth(1),
        false => WalkDir::new(config.path()).min_depth(1).max_depth(1),
    }
    .into_iter()
    .filter(|x| x.as_ref().unwrap().metadata().unwrap().is_file())
    .map(|x| File::new(&x.unwrap()))
    .map(|f| (f.name(), f))
    .collect::<HashMap<_, _>>();

    if config.patterns().is_empty() {
        return files
    }
    let mut filtered_files = HashMap::new();
    for (name, file) in files {
        let ext = file.extension();
        if config.patterns().contains(&ext) {
            filtered_files.insert(name, file);
        }
    }
    filtered_files
}

impl Observer {
    pub fn new(config: Config) -> Self {
        Observer {
            files: get_files(&config),
            config,
        }
    }

    pub fn iter_events(&mut self) -> impl Iterator<Item = FileEvent> + '_ {
        let interval = Duration::from_millis(500);
        let last_files = self.files.clone();
        std::iter::from_fn(move || {
            let current_files = get_files(&self.config);

            let mut events = Vec::new();
            for (name, file) in current_files.iter() {
                if let Some(last_file) = last_files.get(name) {
                    if file.last_modification() > last_file.last_modification() {
                        events.push(FileEvent::Modified(file.clone()));
                    }
                } else {
                    events.push(FileEvent::Created(file.clone()));
                }
            }
            for (name, last_file) in last_files.iter() {
                if !current_files.contains_key(name) {
                    events.push(FileEvent::Eliminated(last_file.clone()));
                }
            }
            std::thread::sleep(interval);
            if !events.is_empty() {
                self.files = current_files;
                Some(events.remove(0))
            } else {
                None
            }
        })
    }
}
