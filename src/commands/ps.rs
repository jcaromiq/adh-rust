use shiplift::Docker;
use tokio::prelude::Future;
use prettytable::Table;

pub fn execute() {
    let docker = Docker::new();
    let fut = docker
        .containers()
        .list(&Default::default())
        .map(|containers| {
            print(containers);
        })
        .map_err(|e| eprintln!("Error: {}", e));

    tokio::run(fut);
}

fn print(containers: Vec<shiplift::rep::Container>) {
    let mut table = Table::new();

    table.add_row(row!["CONTAINER ID", "NAMES", "IMAGE", "STATUS", "PORTS"]);
    for c in containers {
        let mut image = c.image.to_string();
        if image.contains("sha256:") {
            println!("si!");
            image = c.image[7..19].to_string();
        }
        if !c.ports.is_empty() {
            let port = format!("0.0.0.0:{}->{}/{}", c.ports[0].public_port.unwrap(), c.ports[0].private_port, c.ports[0].typ);
            table.add_row(row![&c.id[0..12], c.names[0], image, c.status, &port]);
        } else {
            table.add_row(row![&c.id[0..12], c.names[0], image, c.status, ""]);
        }
    }
    table.printstd();
}