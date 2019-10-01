use std::error::Error;

use shiplift::{ContainerOptions, Docker};
use tokio::prelude::Future;

pub fn create_and_run(options: &ContainerOptions) {
    let docker = Docker::new();
    let fut = docker
        .containers()
        .create(options)
        .and_then(move |nginx| {
            start(&nginx.id);
            Ok(nginx.id)
        })
        .map(|id| println!("docker {:?} created", id))
        .map_err(|e: shiplift::errors::Error| eprintln!("{}", e.description()));
    tokio::run(fut);
}

fn start(id: &String) {
    let docker = Docker::new();
    let start_operation = docker.containers().get(&id).start()
        .map(|_| ())
        .map_err(|e| eprintln!("{:?}", e));
    tokio::spawn(start_operation);
}
