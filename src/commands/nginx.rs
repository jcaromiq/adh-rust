use shiplift::{Docker, ContainerOptions};
use tokio::prelude::Future;

pub fn execute() {
    //FIX: si el contenedor existe pero esta parado, no lo puede crear y tampoco lo arranca, hay que controlar el error
    // y en el caso de que el contenedor este parado, arrancarlo
    let options = &ContainerOptions::builder("nginx")
        .name("adh-nginx")
        .expose(80, "tcp", 8888).build();
    let docker = Docker::new();
    let fut = docker
        .containers()
        .create(options)
        .and_then(move |i| {
            docker.containers().get(&i.id).start()
        })
        .map(|info| println!("docker nginx created with id {:?}", info))
        .map_err(|e| eprintln!("Error creating docker nginx: {}", e));
    tokio::run(fut);
}