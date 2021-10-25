use shiplift::ContainerOptions;

use crate::commands::create_and_run::{create_and_run, create_and_run_from_repo};
use async_trait::async_trait;
use std::env;
use crate::commands::Command;

pub struct Elastic;

#[async_trait]
impl Command for Elastic {
    async fn execute(&self) {
        let mut env = Vec::new();
        env.push("discovery.type=single-node");
        let options = &ContainerOptions::builder("elastic")
            .name("adh-elastic")
            .expose(9200, "tcp", 9200)
            .expose(9300, "tcp", 9300)
            .env(env)
            .build();
        create_and_run_from_repo(options, "docker.elastic.co/elasticsearch/elasticsearch:7.14.0", "elasticsearch", "7.14.0").await;
    }
}
