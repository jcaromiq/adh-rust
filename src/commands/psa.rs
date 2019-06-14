use shiplift::{Docker, ContainerListOptions};
use tokio::prelude::Future;
use crate::domain::container;

pub fn execute() {
    let docker = Docker::new();
    let fut = docker
        .containers()
        .list(&ContainerListOptions::builder().all().build())
        .map(|containers| {
            container::to_domain(containers).print();
        })
        .map_err(|e| eprintln!("Error: {}", e));

    tokio::run(fut);
}