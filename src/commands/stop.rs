use shiplift::Docker;
use tokio::prelude::Future;

use crate::commands::command::Command;

pub struct Stop { pub container_id: String }

impl Command for Stop {
    fn execute(&self) {
        let docker = Docker::new();
        let operation = docker
            .containers()
            .get(&self.container_id)
            .stop(None)
            .map(|_| println!("Container stopped!"))
            .map_err(|e| eprintln!("Error: {}", e));
        tokio::run(operation);
    }
}