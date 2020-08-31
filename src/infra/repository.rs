use shiplift::{ContainerFilter, ContainerListOptions, Docker, RmContainerOptions};

use crate::domain::container;
use crate::domain::container::Containers;

pub async fn get_all_containers() -> Containers {
    get_containers(ContainerListOptions::builder().all().build()).await
}

pub async fn get_exited_containers() -> Containers {
    let mut status = Vec::new();
    status.push(ContainerFilter::Status(String::from("exited")));
    get_containers(ContainerListOptions::builder().filter(status).build()).await
}


pub async fn delete(containers: Containers) {
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

pub async fn get_running_containers() -> Containers {
    let mut status = Vec::new();
    status.push(ContainerFilter::Status(String::from("running")));
    get_containers(ContainerListOptions::builder().filter(status).build()).await
}

pub async fn stop_running_containers() {
    let containers = get_running_containers().await;
    let docker = Docker::new();
    for c in containers.list {
        print!("Killing container with id {}... ",c.id);
        match docker.containers().get(c.id.as_str()).kill(None).await {
            Ok(_) => println!("killed"),
            Err(e) => eprintln!("Error: {} deleting killing container {}", e, c.id),
        }
    }
}

async fn get_containers(filter: ContainerListOptions) -> Containers {
    let docker = Docker::new();
    let containers = match docker
        .containers()
        .list(&filter)
        .await {
        Ok(containers) => containers,
        Err(e) => {
            eprintln!("Error: {}", e);
            vec![]
        }
    };
    container::to_domain(containers)
}

