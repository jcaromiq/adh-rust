use shiplift::{ContainerListOptions, Docker};
use tokio::prelude::Future;

use crate::commands::command::Command;
use crate::domain::container;
use crate::infra::printer;

pub struct Psa {}

impl Psa {}

impl Command for Psa {
    fn new() -> Self {
        Psa {}
    }

    fn execute(&self) {
        let docker = Docker::new();
        let operation = docker
            .containers()
            .list(&ContainerListOptions::builder().all().build())
            .map(|c| container::to_domain(c))
            .map_err(|e| eprintln!("Error: {}", e))
            .and_then(|c| Ok(printer::print(c)));
        tokio::run(operation);
    }
}