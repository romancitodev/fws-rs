use crate::{config::Config, events::EventFiles, file::File};
use std::collections::HashMap;
use std::time::Duration;
use walkdir::WalkDir;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Observer {
    config: Config,
    files: HashMap<String, File>,
}
impl Observer {
    pub fn new(config: Config) -> Self {
        let files = match config.is_recursive() {
            true => WalkDir::new(&config.path()).min_depth(1),
            false => WalkDir::new(&config.path()).min_depth(1).max_depth(1),
        }
        .into_iter()
        .filter(|x| x.as_ref().unwrap().metadata().unwrap().is_file())
        .map(|x| File::new(&x.unwrap()))
        .map(|f| (f.name(), f))
        .collect::<HashMap<_, _>>();
        Observer { config, files }
    }

    pub fn iter_events(&self) -> impl Iterator<Item = EventFiles> + '_ {
        let interval = Duration::from_millis(500);
        let mut last_files = self.files.clone();
        std::iter::from_fn(move || {
            let current_files = match self.config.is_recursive() {
                true => WalkDir::new(self.config.path()).min_depth(1),
                false => WalkDir::new(self.config.path()).min_depth(1).max_depth(1),
            }
            .into_iter()
            .filter(|x| x.as_ref().unwrap().metadata().unwrap().is_file())
            .map(|x| File::new(&x.unwrap()))
            .map(|f| (f.name(), f))
            .collect::<HashMap<String, File>>();
            let mut events = Vec::new();
            for (name, file) in current_files.iter() {
                if let Some(last_file) = last_files.get(name) {
                    if file.last_modification() > last_file.last_modification() {
                        events.push(EventFiles::Modified(file.clone()));
                    }
                } else {
                    events.push(EventFiles::Created(file.clone()));
                }
            }
            for (name, last_file) in last_files.iter() {
                if !current_files.contains_key(name) {
                    events.push(EventFiles::Eliminated(last_file.clone()));
                }
            }
            println!("{:?}", events);
            std::thread::sleep(interval);
            if !events.is_empty() {
                last_files = current_files;
                Some(events.remove(0))
            } else {
                None
            }
        })
    }
}
