use shiplift::Docker;
use tokio::prelude::Future;

pub fn execute(id: &str) {
    let docker = Docker::new();
    let operation = docker
        .containers()
        .get(&id)
        .start()
        .map(|_| println!("Container started!"))
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(operation);
}