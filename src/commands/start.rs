use async_trait::async_trait;
use shiplift::Docker;

use crate::commands::command::Command;
use crate::infra::container_selector::select_container;
use crate::infra::container_repository::get_exited_containers;

pub struct Start {
    pub container_id: Option<String>,
}

#[async_trait]
impl Command for Start {
    async fn execute(&self) {
       let id =  match &self.container_id {
            None =>  select_container(get_exited_containers().await),
            Some(id) => id.to_string()
        };

        let docker = Docker::new();
        match docker.containers().get(id.as_str()).start().await {
            Ok(_) => println!("Container started!"),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
