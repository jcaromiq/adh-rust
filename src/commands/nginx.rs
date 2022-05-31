use shiplift::ContainerOptions;

use crate::commands::create_and_run::create_and_run;
use crate::commands::Command;
use async_trait::async_trait;
use std::env;

pub struct Nginx;

#[async_trait]
impl Command for Nginx {
    async fn execute(&self) {
        let volume = format!(
            "{}:/usr/share/nginx/html",
            env::current_dir().unwrap().display()
        );
        let options = &ContainerOptions::builder("nginx")
            .name("adh-nginx")
            .expose(80, "tcp", 8888)
            .volumes(vec![&volume])
            .build();
        create_and_run(options, "nginx:latest").await;
    }
}
