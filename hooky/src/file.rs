use std::fs::Metadata;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use walkdir::DirEntry;

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

    pub fn path(&self) -> String {
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
}

impl FileData {
    pub fn new(metadata: Metadata) -> Self {
        FileData {
            last_accesed: metadata.accessed().unwrap(),
            last_modified: metadata.modified().unwrap(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FileEvent {
    Created(File),
    Modified(File),
    Eliminated(File),
}
