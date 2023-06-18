use serde::Deserialize;
use std::env;
use std::io::BufReader;
use std::str::FromStr;
use std::{
    collections::HashMap,
    fs::File as FsFile,
    io::{self, Write},
    path::PathBuf,
    time::Duration,
};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use walkdir::WalkDir;

use crate::{
    command::{Command, CommandError, CommandErrorKind},
    config::{SupervisorConfig, WatcherConfig},
    file::{File, FileEvent},
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Errors {
    InvalidParseError,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Interval {
    Check(Duration),
    None,
}

/// 5m -> 3600s
/// 500ms -> 0.5s
/// 5s -> 5s
impl FromStr for Interval {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number_end = s.bytes().take_while(|&b| b.is_ascii_digit()).count();
        let (time, expr) = s.split_at(number_end);
        if let Ok(time) = time.parse::<u64>() {
            let duration = match expr {
                "ms" => Duration::from_millis(time),
                "s" => Duration::from_secs(time),
                "m" => Duration::from_secs(time * 60),
                _ => return Err(Errors::InvalidParseError),
            };
            Ok(Interval::Check(duration))
        } else {
            Err(Errors::InvalidParseError)
        }
    }
}

impl<'de> Deserialize<'de> for Interval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.parse::<Self>().unwrap())
    }
}

pub struct Supervisor {
    watcher: Watcher,
    commands: Vec<Command>,
    sender: Sender<()>,
    receiver: Receiver<()>,
}

impl Supervisor {
    pub fn new(config: SupervisorConfig) -> Self {
        let (tx, rx) = channel(1);
        Self {
            watcher: Watcher::new(config.watcher_config),
            commands: config
                .commands
                .iter()
                .map(|f| f.parse::<Command>().unwrap())
                .collect(),
            sender: tx,
            receiver: rx,
        }
    }

    /// va a iniciar todos los comandos + cargar toda su configuracion
    pub async fn start(&mut self) {
        self.watcher.start();
        self.execute_commands();
        loop {
            tokio::select! {
                _ = async {} => {
                    for command in &mut self.commands {
                        command.wait_async().await;
                    }
                    let event = self.watcher.get_events().next();
                    match event {
                        Some(FileEvent::Created(file)) => {
                            flush_console();
                            println!("watcher detected new file: {:?}", file.path())
                        }
                        Some(FileEvent::Modified(file)) => {
                            flush_console();
                            println!("watcher detected changes in: {:?}", file.path());
                            let path = get_config_file().unwrap();
                            let reader = BufReader::new(FsFile::open(path).unwrap());
                            let supervisor_config: SupervisorConfig = serde_json::from_reader(reader).unwrap();
                            self.reconfigure(supervisor_config);
                            self.execute_commands();
                        }
                        Some(FileEvent::Eliminated(file)) => {
                            flush_console();
                            println!("watcher detected new deleted file: {:?}", file.path())
                        }
                        None => {}
                    }
                }
                _ = self.receiver.recv() => {
                    self.stop().await;
                    break;
                },
            }
        }
    }

    pub async fn listen(&mut self) {
        use tokio::signal::ctrl_c;
        ctrl_c().await.expect("failed to listen for event");
        self.sender.send(()).await.expect("signal cannot be sended");
    }

    /// va a parar todos los comandos que tenga corriendo.
    async fn stop(&mut self) {
        for command in &mut self.commands {
            flush_console();
            println!("killing: {}", command);
            match command.kill() {
                Ok(_) => (),
                Err(CommandError {
                    kind: CommandErrorKind::ExecutionFinalizated,
                    msg: _,
                }) => continue,
                Err(err) => {
                    println!("error: {}", err)
                }
            };
        }
    }

    fn reconfigure(&mut self, config: SupervisorConfig) {
        for command in &mut self.commands {
            flush_console();
            println!("killing: {:?}", command.name());
            match command.kill() {
                Ok(_) => (),
                Err(err) => {
                    println!("error... {:?}", err)
                }
            }
        }
        self.watcher = Watcher::new(config.watcher_config);
        self.commands = config
            .commands
            .iter()
            .map(|f| f.parse::<Command>().unwrap())
            .collect();
        self.watcher.start();
    }

    fn execute_commands(&mut self) {
        for command in &mut self.commands {
            println!("executing: {} ", command);
            command.execute()
        }
    }
}

pub fn get_config_file() -> Option<PathBuf> {
    let root = env::current_dir().unwrap();
    WalkDir::new(root)
        .into_iter()
        .find(|x| x.as_ref().unwrap().file_name().to_str() == Some("watch.json"))
        .map(|file| file.unwrap().into_path())
}
type Files = HashMap<String, File>;

#[allow(dead_code)]
pub struct Watcher {
    path: PathBuf,
    recursive: bool,
    files: Option<Files>,
    interval: Interval,
    patterns: Vec<String>,
}

impl Watcher {
    pub fn new(config: WatcherConfig) -> Self {
        Self {
            files: None,
            path: config.path,
            recursive: config.recursive,
            interval: config.interval,
            patterns: config.patterns,
        }
    }

    pub fn start(&mut self) {
        self.files = Some(self.get_files());
        let a_number = Option::Some(10);
        match a_number {
            Some(x) if x <= 5 => println!("0 to 5 num = {x}"),
            Some(x @ 6..=10) => println!("6 to 10 num = {x}"),
            None => panic!(),
            // all other numbers
            _ => panic!(),
        }
    }

    /// ## `[get_events]`
    /// 1. tendria que ser un iterador de eventos
    /// 2. tendria que ser usada
    pub fn get_events(&mut self) -> impl Iterator<Item = FileEvent> + '_ {
        let last_files = self.files.clone().unwrap();
        std::iter::from_fn(move || {
            self.files.as_ref()?;
            if let Interval::Check(duration) = self.interval {
                std::thread::sleep(duration);
            }
            let current_files = self.get_files();

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
            if !events.is_empty() {
                self.files = Some(current_files);
                Some(events.remove(0))
            } else {
                None
            }
        })
    }

    fn get_files(&self) -> Files {
        let files = match self.is_recursive() {
            true => WalkDir::new(self.path()).min_depth(1),
            false => WalkDir::new(self.path()).min_depth(1).max_depth(1),
        }
        .into_iter()
        .filter(|x| x.as_ref().unwrap().metadata().unwrap().is_file())
        .map(|x| File::new(&x.unwrap()))
        .map(|f| (f.name(), f))
        .collect::<HashMap<_, _>>();
        if self.patterns().is_empty() {
            return files;
        }
        let mut filtered_files = HashMap::new();
        for (name, file) in files {
            let ext = file.extension();
            if self.patterns().contains(&ext) {
                filtered_files.insert(name, file);
            }
        }
        filtered_files
    }

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

fn flush_console() {
    io::stdout().flush().unwrap();
}
