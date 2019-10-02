use std::error::Error;

use shiplift::{ContainerOptions, Docker};
use tokio::prelude::Future;

pub fn create_and_run(options: &ContainerOptions) {
    //TODO: container can exists but maybe is stoped, actually can not create and also can not started
    // need handle this case and start it
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

fn start(id: &str) {
    let docker = Docker::new();
    let start_operation = docker.containers().get(&id).start()
        .map(|_| ())
        .map_err(|e| eprintln!("{:?}", e));
    tokio::spawn(start_operation);
}
