use async_trait::async_trait;
use shiplift::rep::Container;
use shiplift::{ContainerListOptions, Docker, RmContainerOptions, ContainerFilter};

use crate::commands::command::Command;

pub struct RemoveExitedContainers;

#[async_trait]
impl Command for RemoveExitedContainers {
    async fn execute(&self) {
        let mut vec = Vec::new();
        vec.push(ContainerFilter::Status(String::from("exited")));
        let docker = Docker::new();
        match docker
            .containers()
            .list(&ContainerListOptions::builder().filter(vec).build())
            .await
        {
            Ok(container) => delete(container).await,
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

async fn delete(containers: Vec<Container>) {
    let docker = Docker::new();
    for container in containers {
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
