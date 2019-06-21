use shiplift::Docker;
use tokio::prelude::Future;
use crate::infra::printer;
use crate::domain::container;

pub fn execute() {
    let docker = Docker::new();
    let operation = docker
        .containers()
        .list(&Default::default())
        .map(|c| container::to_domain(c))
        .map_err(|e| eprintln!("Error: {}", e))
        .and_then(|c| Ok(printer::print(c)));
    tokio::run(operation);
}