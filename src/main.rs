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
    } = CommandArguments::from_args();
    if let Err(err) = check_arguments(&path, &exe, &config) {
        println!("{:?}", err);
        exit(1);
    }
    let observer_config = config.map_or_else(
        || Config::load_from_args(path.unwrap(), exe.unwrap(), recursive),
        |path| Config::load_from_file(&path).unwrap(),
    );

    let observer = Observer::new(observer_config.clone());
    println!("🤖 Starting observer...");
    loop {
        match observer.iter_events().next() {
            Some(EventFiles::Created(file)) => println!("📁 New file detected: {:?}", file.name()),
            Some(EventFiles::Modified(file)) => println!(
                "📑 Changes on file: {:?}\n🚀 Executing: {}",
                file.ds_path(),
                observer_config.clone().exec()
            ),
            Some(EventFiles::Eliminated(file)) => println!("🗑️ Removed file: {:?}", file.name()),
            None => {}
        }
    }
}
