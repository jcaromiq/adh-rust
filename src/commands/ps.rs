use shiplift::Docker;
use tokio::prelude::Future;

use crate::commands::command::Command;
use crate::domain::container;
use crate::infra::printer;

pub struct Ps;

impl Command for Ps {
    fn execute(&self) {
        let docker = Docker::new();
        let operation = docker
            .containers()
            .list(&Default::default())
            .map(container::to_domain)
            .map_err(|e| eprintln!("Error: {}", e))
            .and_then(|c| {
                printer::print(c);
                Ok(())
            });
        tokio::run(operation);
    }
}
