use shiplift::{ContainerOptions, Docker};
use tokio::prelude::Future;

use crate::commands::command::Command;
use std::error::Error;

pub struct Nginx;

impl Command for Nginx {
    fn execute(&self) {
        //FIX: si el contenedor existe pero esta parado, no lo puede crear y tampoco lo arranca, hay que controlar el error
        // y en el caso de que el contenedor este parado, arrancarlo
        let options = &ContainerOptions::builder("nginx")
            .name("adh-nginx")
            .expose(80, "tcp", 8888).build();
        let docker = Docker::new();
        let fut = docker
            .containers()
            .create(options)
            .and_then(move |nginx| {
                docker.containers().get(&nginx.id).start();
                Ok(nginx)
            })
            .map(|info| println!("docker nginx created with id {:?}", info.id))
            .map_err(|e:shiplift::errors::Error| eprintln!("{}", e.description()));
        tokio::run(fut);
    }
}
