use shiplift::{Docker};
use tokio::prelude::Future;
#[macro_use] extern crate prettytable;
use prettytable::{Table};

struct ContainerInfo {
    id: String,
    name: String,
    image: String,
    status: String,
    ports: String,
}

fn main() {
    let docker = Docker::new();
    let fut = docker
        .containers()
        .list(&Default::default())
        .map(|containers| {
            let mut table = Table::new();

            table.add_row(row!["CONTAINER ID", "NAMES", "IMAGE", "STATUS", "PORTS"]);
            for c in containers {
                let p = format!("0.0.0.0:{}->{}/{}",c.ports[0].public_port.unwrap(),c.ports[0].private_port,c.ports[0].typ);
                table.add_row(row![c.id, c.names[0], c.image, c.status, p]);

            }
            table.printstd();
        })
        .map_err(|e| eprintln!("Error: {}", e));

    tokio::run(fut);


}