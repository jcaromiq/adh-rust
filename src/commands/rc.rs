use shiplift::{Docker, Error, RmContainerOptions, ContainerListOptions};
use tokio::prelude::Future;
use shiplift::rep::Container;

pub fn execute() {
    let docker = Docker::new();
    let delete_operation = docker
        .containers()
        .list(&ContainerListOptions::builder().all().build())
        .and_then(move |containers| {
            delete(containers)
        })
        .map(|info| eprintln!("{:?}", info))
        .map_err(|e| eprintln!("Error {}", e));

    tokio::run(delete_operation);
}

fn delete(containers: Vec<Container>) -> std::result::Result<(), Error> {
    let docker = Docker::new();
    for container in containers {
        let ff = docker.containers()
            .get(container.id.as_str())
            .remove(RmContainerOptions::builder().force(true).build())
            .map(move |_| println!("deleted {}", container.id))
            .map_err(|e| eprintln!("Error: {} deleting container", e));
        tokio::spawn(ff);
    }
    Ok(())
}