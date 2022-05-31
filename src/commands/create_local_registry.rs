use shiplift::ContainerOptions;

use crate::commands::create_and_run::create_and_run;
use crate::commands::Command;

pub struct LocalRegistry;
use async_trait::async_trait;

#[async_trait]
impl Command for LocalRegistry {
    async fn execute(&self) {
        let options = &ContainerOptions::builder("registry:2")
            .name("local-registry")
            .expose(5000, "tcp", 5000)
            .build();
        create_and_run(options, "registry:2").await;
    }
}
