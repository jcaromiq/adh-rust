use async_trait::async_trait;
use shiplift::{ContainerListOptions, Docker};

use crate::commands::command::Command;
use crate::domain::container;
use crate::infra::printer;

pub struct Psa;

#[async_trait]
impl Command for Psa {
    async fn execute(&self) {
        let docker = Docker::new();
        match docker
            .containers()
            .list(&ContainerListOptions::builder().all().build())
            .await
        {
            Ok(containers) => printer::print(container::to_domain(containers)),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
