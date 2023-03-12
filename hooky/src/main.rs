use hooky::{config::SupervisorConfig, watcher::Supervisor};
use std::fs::File;
use std::io::BufReader;

#[tokio::main]
async fn main() {
    let file = File::open("fws-tests/watch.json").unwrap();
    let reader = BufReader::new(file);
    let supervisor_config: SupervisorConfig = serde_json::from_reader(reader).unwrap();
    let mut observer: Supervisor = Supervisor::new(supervisor_config);
    println!("observer started");
    tokio::select! {
        _ = &observer.listen() =>{},
        _ = &observer.start() =>{},
    }
}
