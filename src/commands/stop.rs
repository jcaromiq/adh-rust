use async_trait::async_trait;
use shiplift::Docker;

use crate::commands::Command;
use crate::infra::container_repository::get_running_containers;
use crate::infra::container_selector::select_container;

pub struct Stop {
    pub container_id: Option<String>,
}

#[async_trait]
impl Command for Stop {
    async fn execute(&self) {
        let id = match &self.container_id {
            None => select_container(get_running_containers().await),
            Some(id) => Some(id.to_string()),
        };
        match id {
            None => {
                println!("No containers found");
            }
            Some(container_id) => {
                let docker = Docker::new();
                match docker
                    .containers()
                    .get(container_id.as_str())
                    .stop(None)
                    .await
                {
                    Ok(_) => println!("Container stopped!"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}
