use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(
    name = "Observe",
    about = "A file watcher system to detect changes in your project.",
    rename_all = "kebab-case"
)]
#[serde(rename_all = "kebab-case")]
#[allow(dead_code)]
pub struct CommandArguments {
    #[arg(short, long)]
    pub watch: Option<PathBuf>,
    #[arg(short, long)]
    pub exec: Option<String>,
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    #[arg(short, long, help = "Set the recursion to True (default False)")]
    pub recursive: bool,
    #[arg(
        short,
        long,
        help = "Execute the command only on events (default False)"
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
