use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::config::get_config_file;

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(
    name = "Observe",
    about = "A file watcher system to detect changes in your project.",
    rename_all = "kebab-case"
)]
#[serde(rename_all = "kebab-case")]
pub struct Args {
    #[arg(short, long)]
    pub watch: Option<PathBuf>,
    #[arg(short, long)]
    pub exec: Option<String>,
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    #[arg(short, long, help = "Set the recursion to True (default False)")]
    pub non_recursive: bool,
    #[arg(
        short,
        long,
        help = "Execute the command only on events (default True)"
    )]
    pub on_events_only: bool,
    #[arg(
        short,
        long,
        help = "Set the max attempts to retry the command in case of fails",
        default_value_t = 3
    )]
    pub attempts: usize,
    #[arg(
        short,
        long,
        default_values_t = Vec::<String>::new(),
        help = "the patterns to observe. Example: [rs, ts, jsx]"
    )]
    pub patterns: Vec<String>,
}


#[derive(Debug)]
pub enum ArgsErrorKind {
    UnexpectedArgument,
    MissingArgument,
}

pub struct ArgsError {
    pub kind: ArgsErrorKind,
    pub msg: String
}

pub fn check_arguments(args: Args) -> Result<Args, ArgsError> {
    match (&args.config, &args.exec, &args.watch) {
        (Some(_),Some(_) , _) => return Err(ArgsError {kind: ArgsErrorKind::UnexpectedArgument, msg: "Invalid argument if --config is active (exec)".into()}),
        (Some(_), _, Some(_)) => return Err(ArgsError {kind: ArgsErrorKind::UnexpectedArgument, msg: "Invalid argument if --config is active (watch)".into()}),
        (None, None, Some(_)) => return Err(ArgsError {kind: ArgsErrorKind::MissingArgument, msg: "Missing argument (exec)".into()}),
        (None, Some(_), None) => return Err(ArgsError {kind: ArgsErrorKind::MissingArgument, msg: "Missing argument (watch)".into()}),
        (None, None, None) => return Ok(Args {
            config: Some(get_config_file().unwrap()),
            watch: None,
            exec: None,
            non_recursive: args.non_recursive,
            on_events_only: args.on_events_only,
            attempts: !args.attempts,
            patterns: args.patterns
        }),
        (Some(_), None, None) | (None, Some(_), Some(_)) => return Ok(args)
    };
}