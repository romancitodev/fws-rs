use hooky::{
    config::SupervisorConfig,
    watcher::{get_config_file, Supervisor},
};
use std::{fs::File, io::BufReader};

#[tokio::main]
async fn main() {
    env_logger::init(); // initialize the logger
    let file = File::open(get_config_file().unwrap()).unwrap();
    let reader = BufReader::new(file);
    let supervisor_config: SupervisorConfig = serde_json::from_reader(reader).unwrap();
    let mut observer: Supervisor = Supervisor::new(supervisor_config);
    log::info!("starting supervisor...");
    println!("starting supervisor...");
    // tokio::select! {
    //     _ = observer.listen() => {},
    //     _ = observer.start() => {}
    // }
    // observer.listen().await;
    observer.start().await;
}
