mod error;
mod cli;
mod structs;
mod events;
mod file;

use structs::Config;
use file::File;
use cli::CommandArguments;
use error::ArgumentsError;

use walkdir::WalkDir;
use structopt::StructOpt;
use std::{io, process::exit, path::PathBuf};

pub fn check_arguments(path: &Option<PathBuf>, exe: &Option<String>, config: &Option<PathBuf>) -> Result<(), ArgumentsError> {
    match (path.is_some(), exe.is_some(), config.is_some()) {
        (true, _, true) | (_, true, true) => Err(ArgumentsError::UnexpectedCommands),
        (false, true, _) | (true, false, _) | (false, false, false) => Err(ArgumentsError::MissingCommands),
        (_, _, _) => Ok(())
    }
}

fn main() -> Result<(), io::Error>{
    let CommandArguments {
        watch: path,
        exec: exe,
        config,
        recursive
    } = CommandArguments::from_args();
    if let Err(err) = check_arguments(&path, &exe, &config) {
        println!("{:?}", err);
        exit(1);
    }

    let command_config = config.map_or_else(
    || Config::load_from_args(path.unwrap(), exe.unwrap(), recursive),
    |path| Config::load_from_file(&path).unwrap());


    let files = match &command_config.is_recursive() {
        true => WalkDir::new(&command_config.path()).min_depth(1),
        false => WalkDir::new(&command_config.path()).min_depth(1).max_depth(1)
    };

    let files = files.into_iter().filter(
    |x| x.as_ref().unwrap().metadata().unwrap().is_file()
    ).map(|x| File::new(&x.unwrap())).collect::<Vec<_>>();

    files.into_iter().for_each(|f| println!("{:#?}\n", f));
    println!("{command_config:?}");

    Ok(())
}
