mod cli;
mod config;
mod error;
mod events;
mod file;
mod observer;

use cli::CommandArguments;
use config::Config;
use error::ArgumentsError;
use events::EventFiles;
use observer::Observer;
use std::{path::PathBuf, process::exit};
use structopt::StructOpt;
use subprocess::{Exec, Popen};

pub fn check_arguments(
    path: &Option<PathBuf>,
    exe: &Option<String>,
    config: &Option<PathBuf>,
) -> Result<(), ArgumentsError> {
    match (path.is_some(), exe.is_some(), config.is_some()) {
        (true, _, true) | (_, true, true) => Err(ArgumentsError::UnexpectedCommands),
        (false, true, _) | (true, false, _) | (false, false, false) => {
            Err(ArgumentsError::MissingCommands)
        }
        (_, _, _) => Ok(()),
    }
}

fn main() {
    let CommandArguments {
        watch: path,
        exec: exe,
        config,
        recursive,
        on_events_only
    } = CommandArguments::from_args();
    if let Err(err) = check_arguments(&path, &exe, &config) {
        println!("{:?}", err);
        exit(1);
    }
    let observer_config = config.map_or_else(
        || Config::load_from_args(path.unwrap(), exe.clone().unwrap(), recursive, on_events_only),
        |path| Config::load_from_file(&path).unwrap(),
    );

    let mut observer = Observer::new(observer_config.clone());
    println!("🤖 Starting observer...");
    let mut cmd:Option<Popen> = None;

    if !observer_config.reload_on_events() {
        let _cmd = Some(Exec::shell(observer_config.exec()).popen().unwrap());
    }

    loop {
        match observer.iter_events().next() {
            Some(EventFiles::Created(file)) => {
                println!("📁 New file detected: {:?}", file.ds_path())
            }
            Some(EventFiles::Modified(file)) => {
                println!(
                    "📑 Changes on file: {:?}\n🚀 Executing: {}",
                    file.ds_path(),
                    observer_config.clone().exec()
                );
                if let Some(mut command) = cmd {
                    command.kill().unwrap();
                }
                cmd = Some(Exec::shell(observer_config.exec()).popen().unwrap());
            }
            Some(EventFiles::Eliminated(file)) => println!("🗑️ Removed file: {:?}", file.ds_path()),
            None => {}
        }
    }
}
