use shiplift::Docker;
use tokio::prelude::Future;

use crate::commands::command::Command;

pub struct Start { pub container_id: String }

impl Command for Start {
    fn execute(&self) {
        let docker = Docker::new();
        let operation = docker
            .containers()
            .get(&self.container_id)
            .start()
            .map(|_| println!("Container started!"))
            .map_err(|e| eprintln!("Error: {}", e));
        tokio::run(operation);
    }
}
