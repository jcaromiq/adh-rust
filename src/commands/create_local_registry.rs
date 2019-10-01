use shiplift::{ContainerOptions, Docker};
use tokio::prelude::Future;

use crate::commands::command::Command;
use std::error::Error;

pub struct LocalRegistry;

impl Command for LocalRegistry {
    fn execute(&self) {
        //FIX: si el contenedor existe pero esta parado, no lo puede crear y tampoco lo arranca, hay que controlar el error
        // y en el caso de que el contenedor este parado, arrancarlo
        let options = &ContainerOptions::builder("registry:2")
            .name("local-registry")
            .expose(5000, "tcp", 5000).build();
        let docker = Docker::new();
        let fut = docker
            .containers()
            .create(options)
            .and_then(move |registry| {
                docker.containers().get(&registry.id).start()
            })
            //TODO: print container id
            .map(|_| println!("docker registry created"))
            .map_err(|e:shiplift::errors::Error| eprintln!("{}", e.description()));
        tokio::run(fut);
    }
}

