use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[structopt(
    name = "Observe",
    about = "A file watcher system to detect changes in your project.",
    rename_all="kebab-case"
)]
#[serde(rename_all="kebab-case")]
#[allow(dead_code)]
pub struct CommandArguments {
    #[structopt(short, long, parse(from_os_str))]
    pub watch: Option<PathBuf>,
    #[structopt(short, long)]
    pub exec: Option<String>,
    #[structopt(short, long, parse(from_os_str))]
    pub config: Option<PathBuf>,
    #[structopt(short, long, help = "Set the recursion to True (default False)")]
    pub recursive: bool,
    #[structopt(short, long, help = "Execute the command only on events (default False)")]
    pub on_events_only: bool,
}
