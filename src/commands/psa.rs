use shiplift::{ContainerListOptions, Docker};
use tokio::prelude::Future;

use crate::commands::command::Command;
use crate::domain::container;
use crate::infra::printer;

pub struct Psa;

impl Command for Psa {
    fn execute(&self) {
        let docker = Docker::new();
        let operation = docker
            .containers()
            .list(&ContainerListOptions::builder().all().build())
            .map(container::to_domain)
            .map_err(|e| eprintln!("Error: {}", e))
            .and_then(|c| Ok(printer::print(c)));
        tokio::run(operation);
    }
}
