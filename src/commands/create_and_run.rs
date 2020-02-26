use std::error::Error;

use shiplift::{ContainerOptions, Docker, PullOptions};
use tokio::prelude::{Future, Stream};

pub fn create_and_run(options: &ContainerOptions, image_name: &str) {
    //TODO: container can exists but maybe is stoped, actually can not create and also can not started
    // need handle this case and start it
    let docker = Docker::new();
    let pull = docker
        .images()
        .pull(&PullOptions::builder().image(image_name).build())
        .for_each(|_| {
            Ok(())
        })
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(pull);
    let fut = docker
        .containers()
        .create(options)
        .and_then(move |created| {
            start(&created.id);
            Ok(created.id)
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
