use async_trait::async_trait;
use shiplift::Docker;

use crate::commands::command::Command;

pub struct Start {
    pub container_id: String,
}

#[async_trait]
impl Command for Start {
    async fn execute(&self) {
        let docker = Docker::new();
        match docker.containers().get(&self.container_id).start().await {
            Ok(_) => println!("Container started!"),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
