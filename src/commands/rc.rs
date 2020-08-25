use async_trait::async_trait;
use shiplift::{ContainerListOptions, Docker, Error, RmContainerOptions};
use shiplift::rep::Container;

use crate::commands::command::Command;

pub struct RemoveContainers;

#[async_trait]
impl Command for RemoveContainers {
    async fn execute(&self) {
        let docker = Docker::new();
        match docker
            .containers()
            .list(&ContainerListOptions::builder().all().build())
            .await {
            Ok(container) => delete(container).await,
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

async fn delete(containers: Vec<Container>) {
    let docker = Docker::new();
    for container in containers {
        match docker.containers()
            .get(container.id.as_str())
            .remove(RmContainerOptions::builder().force(true).build())
            .await {
            Ok(_) => println!("deleted container '{}' with id {}", container.image, container.id),
            Err(e) => eprintln!("Error: {} deleting container", e),
        };
    }
}
