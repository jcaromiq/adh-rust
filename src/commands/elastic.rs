use shiplift::ContainerOptions;

use crate::commands::create_and_run::{create_and_run_latest, create_and_run_with_tag};
use crate::commands::Command;
use async_trait::async_trait;

pub struct Elastic;

const VERSION: &str = "8.2.3";
const IMAGE: &str = "elasticsearch";

#[async_trait]
impl Command for Elastic {
    async fn execute(&self) {
        let mut env = Vec::new();
        env.push("discovery.type=single-node");
        let options =
            &ContainerOptions::builder("elasticsearch:8.2.3")
                .name("adh-elastic")
                .expose(9200, "tcp", 9200)
                .expose(9300, "tcp", 9300)
                .env(env)
                .build();
        create_and_run_with_tag(
            options,
            IMAGE,
            VERSION,
        ).await;
    }
}
