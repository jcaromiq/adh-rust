use async_trait::async_trait;
use shiplift::{ Docker, RmContainerOptions};

use crate::commands::command::Command;
use crate::infra::repository::get_exited_containers;
use crate::domain::container::Containers;

pub struct RemoveExitedContainers;

#[async_trait]
impl Command for RemoveExitedContainers {
    async fn execute(&self) {
        let containers = get_exited_containers().await;
        delete(containers).await;
    }
}

async fn delete(containers: Containers) {
    let docker = Docker::new();
    for container in containers.list {
        match docker
            .containers()
            .get(container.id.as_str())
            .remove(RmContainerOptions::builder().force(true).build())
            .await
        {
            Ok(_) => println!(
                "deleted container '{}' with id {}",
                container.image, container.id
            ),
            Err(e) => eprintln!("Error: {} deleting container", e),
        };
    }
}
