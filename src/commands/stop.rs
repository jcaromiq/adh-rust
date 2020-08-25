use shiplift::Docker;
use async_trait::async_trait;

use crate::commands::command::Command;

pub struct Stop { pub container_id: String }

#[async_trait]
impl Command for Stop {
    async fn execute(&self) {
        let docker = Docker::new();
        match docker
            .containers()
            .get(&self.container_id)
            .stop(None)
            .await {
            Ok(_) =>  println!("Container stopped!"),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
