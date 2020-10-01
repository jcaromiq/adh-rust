use async_trait::async_trait;
use shiplift::Docker;

use crate::commands::command::Command;
use crate::infra::container_repository::get_exited_containers;
use crate::infra::container_selector::select_container;

pub struct Start {
    pub container_id: Option<String>,
}

#[async_trait]
impl Command for Start {
    async fn execute(&self) {
        let id = match &self.container_id {
            None => select_container(get_exited_containers().await),
            Some(id) => Some(id.to_string()),
        };
        match id {
            None => { println!("No containers found"); }
            Some(container_id) => {
                let docker = Docker::new();
                match docker.containers().get(container_id.as_str()).start().await {
                    Ok(_) => println!("Container started!"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
    }
}
