use shiplift::Docker;
use tokio::prelude::Future;
use crate::domain::container;

pub fn execute() {
    let docker = Docker::new();
    let fut = docker
        .containers()
        .list(&Default::default())
        .map(|containers| {
            container::to_domain(containers).print();
        })
        .map_err(|e| eprintln!("Error: {}", e));

    tokio::run(fut);
}