use shiplift::ContainerOptions;

use crate::commands::command::Command;
use crate::commands::create_and_run::create_and_run;

pub struct Nginx;

impl Command for Nginx {
    fn execute(&self) {
        let options = &ContainerOptions::builder("nginx")
            .name("adh-nginx")
            .expose(80, "tcp", 8888).build();
        create_and_run(options);
    }
}
