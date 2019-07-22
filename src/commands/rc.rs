use shiplift::{Docker, builder, Error, rep, RmContainerOptions, ContainerListOptions};
use tokio::prelude::Future;
use crate::infra::printer;
use crate::domain::container;
use tokio_buf::util::BufStreamExt;
use futures::future::ok;
use shiplift::builder::RmContainerOptionsBuilder;
use std::time::Duration;

pub fn execute() {
    //let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
    //let containers: Result<Vec<rep::Container>, Error> = runtime.block_on(get_all_containers());
    get_all_containers().wait();
//    match containers {
//        Ok(v) => delete_all_containers(v),
//        _ => { print!("err") }
//    }
}

fn delete_all_containers(containers: Vec<rep::Container>) {
    for x in containers {
        let id: &str = x.id.as_str();
        println!("deleting {}", id);
        stop_container(id).wait().map_err(|e| eprintln!("Error: {}", e));
        println!("stopped {}", id);
        delete_container_by(id).wait();
        println!("deleted {}", id);
    }
}

fn stop_container(id: &str) -> impl Future<Item=(), Error=Error> {
    let docker = Docker::new();
    docker.containers()
        .get(&id)
        .stop(Some(Duration::new(5, 0)))
}

fn delete_container_by(id: &str) -> impl Future<Item=(), Error=Error> {
    let docker = Docker::new();
    docker
        .containers()
        .get(&id)
        .remove(RmContainerOptions::builder().force(true).build())
}

fn get_all_containers() -> impl Future<Item=Vec<rep::Container>, Error=Error> {
    let docker = Docker::new();
    return docker
        .containers()
        .list(&ContainerListOptions::builder().all().build());
}
