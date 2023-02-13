use std::fs::Metadata;
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::DirEntry;

#[allow(dead_code)]
#[derive(Debug)]
pub struct File {
    name: String,
    path: PathBuf,
    data: FileData
}

#[allow(dead_code)]
#[derive(Debug)]
struct FileData {
    last_accesed: SystemTime,
    last_modified: SystemTime
}

#[allow(dead_code)]
impl File {

    pub fn new(file: &DirEntry) -> Self {
        let metadata = file.metadata().unwrap();
        File {
            name: file.file_name().to_str().unwrap().to_string(),
            path: file.path().to_path_buf(),
            data: FileData::new(metadata)
        }
    }

    pub fn was_deleted(&self) -> bool {
        !self.path.exists()
    }
}

impl FileData {
    pub fn new(metadata: Metadata) -> Self {
        FileData { last_accesed: metadata.accessed().unwrap(), last_modified: metadata.modified().unwrap() }
    }
}