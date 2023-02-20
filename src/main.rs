mod cli;
mod config;
mod error;
mod events;
mod file;
mod observer;

use clap::Parser;
use cli::CommandArguments;
use config::Config;
use ctrlc::set_handler;
use error::ArgumentsError;
use events::EventFiles;
use observer::Observer;
use std::{
    error::Error,
    ffi::OsStr,
    io::Write,
    path::PathBuf,
    process::exit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};
use subprocess::{Exec, ExitStatus, Popen};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SHUTDOWN: Arc<(Mutex<bool>, Condvar)> =
        Arc::new((Mutex::new(false), Condvar::new()));
}

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
fn execute(command: &impl AsRef<OsStr>, attempts: usize) -> Result<Popen, String> {
    for i in 1..=attempts {
        let mut cmd = Exec::shell(command).popen().expect("a new process handler");
        match cmd.wait() {
            Ok(status) => {
                if status == ExitStatus::Exited(0) {
                    return Ok(cmd);
                }
            }
            Err(_) => {
                println!("🔄 restarting command... {i}/{attempts}");
            }
        }
        std::thread::sleep(Duration::from_secs(2));
    }
    Err("❌ command cannot be executed correctly, shutting down the process.".into())
}

fn try_execute(command: &impl AsRef<OsStr>, attempts: usize) -> Popen {
    match execute(command, attempts) {
        Ok(cmd) => cmd,
        Err(error) => {
            println!("Error: {error}");
            exit(2);
        }
    }
}
fn flush_console() {
    std::io::stdout().flush().unwrap();
}

fn restart(observer: &Arc<Mutex<Option<JoinHandle<()>>>>, terminate: &Arc<AtomicBool>) {
    let mut observer = observer.lock().unwrap();
    if let Some(thread) = observer.take() {
        thread::sleep(std::time::Duration::from_millis(10));
        thread.join().unwrap();
    }
    let new_observer = create_observer_thread(terminate.clone());
    *observer = Some(new_observer);
    println!("🔄 Observer restarted successfully.");
    terminate.store(false, Ordering::SeqCst);
}

fn create_observer_thread(terminate: Arc<AtomicBool>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let shutdown = SHUTDOWN.clone();
        let CommandArguments {
            watch: path,
            exec: exe,
            config,
            recursive,
            on_events_only,
            attempts,
            patterns,
        } = CommandArguments::parse();

        if let Err(err) = check_arguments(&path, &exe, &config) {
            println!("{:?}", err);
            exit(1);
        }

        let observer_config = config.map_or_else(
            || {
                Config::load_from_args(
                    path.unwrap(),
                    exe.clone().unwrap(),
                    recursive,
                    on_events_only,
                    patterns,
                )
            },
            |path| Config::load_from_file(&path).unwrap(),
        );

        let mut observer = Observer::new(observer_config.clone());
        flush_console();
        println!("🤖 Starting observer...");
         let mut cmd: Option<Popen> = None;
        if !observer_config.only_events() {
            cmd = Some(try_execute(&observer_config.exec(), attempts));
        }
        loop {
            match observer.iter_events().next() {
                Some(EventFiles::Created(file)) => {
                    flush_console();
                    println!("📁 New file detected: {:?}", file.ds_path())
                }
                Some(EventFiles::Modified(file)) => {
                    flush_console();
                    println!(
                        "📑 Changes on file: {:?}\n🚀 Executing: {}",
                        file.ds_path(),
                        observer_config.clone().exec()
                    );
                    if let Some(mut command) = cmd {
                        command.kill().unwrap();
                    } //
                    cmd = Some(try_execute(&observer_config.exec(), attempts));
                }
                Some(EventFiles::Eliminated(file)) => {
                    flush_console();
                    println!("🗑️ Removed file: {:?}", file.ds_path())
                }
                None => {}
            }
            if terminate.load(Ordering::SeqCst) {
                let &(ref lock, ref cvar) = &*shutdown;
                let mut shutdown = lock.lock().unwrap();
                *shutdown = true;
                cvar.notify_all();
                break;
            }
        }
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    set_handler(move || {
        flush_console();
        println!("🔪 Shutting down observer...");
        std::process::exit(0);
    })
    .unwrap();

    let observer_terminate = Arc::new(AtomicBool::new(false));
    let observer = Arc::new(Mutex::new(Some(create_observer_thread(
        observer_terminate.clone(),
    ))));

    loop {
        let mut buffer = String::new();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.trim() {
                "exit" => {
                    observer_terminate.store(true, Ordering::SeqCst);
                    if let Some(thread) = observer.lock().unwrap().take() {
                        thread::sleep(std::time::Duration::from_millis(10));
                        drop(thread);
                    }
                    println!("🔪 Shutting down observer...");
                    exit(0);
                }
                "clear" => {
                    println!("\x1B[2J\x1B[1;1H");
                }
                "restart" => {
                    observer_terminate.store(true, Ordering::SeqCst);
                    restart(&observer, &observer_terminate);
                }
                _ => {
                    println!("Unrecognized command.");
                }
            },
            Err(error) => {
                flush_console();
                println!("error reading command: {error}");
            }
        }
    }
}
