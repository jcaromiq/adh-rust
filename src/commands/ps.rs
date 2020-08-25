use async_trait::async_trait;
use shiplift::Docker;

use crate::commands::command::Command;
use crate::domain::container;
use crate::infra::printer;

pub struct Ps;

#[async_trait]
impl Command for Ps {
    async fn execute(&self) {
        let docker = Docker::new();
        match docker
            .containers()
            .list(&Default::default())
            .await
        {
            Ok(containers) => printer::print(container::to_domain(containers)),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
