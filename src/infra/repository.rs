use shiplift::{ContainerListOptions, Docker, ContainerFilter};
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

pub async fn get_running_containers() -> Containers {
    let mut status = Vec::new();
    status.push(ContainerFilter::Status(String::from("running")));
    get_containers(ContainerListOptions::builder().filter(status).build()).await
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


#[tokio::test]
async fn should_get_all_containers() {
    let v = get_all_containers().await;
    dbg!(v);
}

#[tokio::test]
async fn should_get_stopped_containers() {
    let v = get_exited_containers().await;
    dbg!(v);
}

#[tokio::test]
async fn should_get_running_containers() {
    let v = get_running_containers().await;
    dbg!(v);
}
