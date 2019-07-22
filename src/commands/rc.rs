use shiplift::{Docker, Error, RmContainerOptions, ContainerListOptions};
use tokio::prelude::Future;
use shiplift::rep::{Container, Info};
use futures::future::ok;


pub fn execute() {
    let docker_runtime = Docker::new();

    let f = docker_runtime
        .containers()
        .list(&ContainerListOptions::builder().all().build())
        .and_then(move |containers| {
            for container in containers {
                let ff = docker_runtime.containers()
                    .get(container.id.as_str())
                    .remove(RmContainerOptions::builder().force(true).build())
                    .map(move |_| println!("deleted {}", container.id))
                    .map_err(|e| eprintln!("Error: {} deleting container", e));
                tokio::spawn(ff);
            }
            docker_runtime.info()
        })
        .map(|info| eprintln!("{:?}", info))
        .map_err(|e| eprintln!("Error {}", e));

    tokio::run(f);
}
