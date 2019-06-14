use shiplift::{Docker, ExecContainerOptions, ContainerOptions};
use tokio::prelude::Future;
use crate::domain::container;
use shiplift::tty::StreamType;
use futures::stream::Stream;
use shiplift::builder::EventFilterType::Volume;

pub fn execute() {
    let options = &ContainerOptions::builder("nginx")
        .name("adh-nginx")
        .expose(80, "tcp", 8888).build();
    let docker = Docker::new();
    let fut = docker
        .containers()
        .create(options)
        .and_then(move |i| {
            docker.containers().get(&i.id).start()
        })
        .map(|info| println!("docker nginx created with id {:?}", info))
        .map_err(|e| eprintln!("Error creating docker nginx: {}", e));
    tokio::run(fut);
}