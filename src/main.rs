use shiplift::{Docker, ContainerListOptions};
use tokio::prelude::Future;
use clap::{App, SubCommand};

#[macro_use]
extern crate prettytable;

use prettytable::Table;

struct ContainerInfo {
    id: String,
    name: String,
    image: String,
    status: String,
    ports: String,
}

fn main() {
    let matches = App::new("ApiumDockerHelper")
        .version("0.1.0")
        .author("Joaco <me@joaquin-caro.es>")
        .about("adh")
        .subcommand(SubCommand::with_name("ps").about("Formatted ps for running dockers"))
        .subcommand(SubCommand::with_name("psa").about("Formatted ps for all dockers"))
//        .subcommand(SubCommand::with_name("deploy")
//            .arg(Arg::with_name("serviceName").index(1).required(true))
//            .arg(Arg::with_name("stage").index(2).required(true))
//            .arg(Arg::with_name("version").value_name("version").required(true).short("v")))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("ps") {
        ps(Docker::new());
    }
    if let Some(_) = matches.subcommand_matches("psa") {
        psa(Docker::new());
    }


}

fn print() {
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
        table.printstd();

    }
}
pub fn psa(docker: Docker) {
    let fut = docker
        .containers()
        .list(&ContainerListOptions::builder().all().build())
        .map(|containers| {
            print(containers)

        })
        .map_err(|e| eprintln!("Error: {}", e));

    tokio::run(fut);
}

pub fn ps(docker: Docker) {
    let fut = docker
        .containers()
        .list(&Default::default())
        .map(|containers| {
            let mut table = Table::new();

            table.add_row(row!["CONTAINER ID", "NAMES", "IMAGE", "STATUS", "PORTS"]);
            for c in containers {
                let mut image = c.image.to_string();
                if image.contains("sha256:") {
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
        })
        .map_err(|e| eprintln!("Error: {}", e));

    tokio::run(fut);
}