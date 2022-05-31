#[macro_use]
extern crate prettytable;

use crate::commands::from;
use clap::{App, Arg, SubCommand};

mod commands;
mod domain;
mod infra;
mod utils;

#[tokio::main]
async fn main() {
    let matches = App::new("adh")
        .version("1.2.1")
        .author("Joaco <me@joaquin-caro.es>")
        .about("
        ___    ____  __  __
       /   |  / __ \\/ / / /
      / /| | / / / / /_/ /
     / ___ |/ /_/ / __  /
    /_/  |_/_____/_/ /_/")
        .subcommand(SubCommand::with_name("nginx")
            .about("Run nginx with a volume in the current directory"))
        .subcommand(SubCommand::with_name("mysql")
            .about("Run mysql")
            .arg(Arg::with_name("root_password")
                .help("Specifies the password that will be set for the MySQL root superuser account. If not set, a random password will be created and printed at the end")
                .takes_value(true)
                .long("root_password"))
            .arg(Arg::with_name("database_name")
                .long("database_name")
                .takes_value(true)
                .help("Allows you to specify the name of a database to be created on image startup")
                .required(false)))
        .subcommand(SubCommand::with_name("start")
            .about("Start containers")
            .arg(Arg::with_name("container_id")
                .help("container id to start")
                .required(false)))
        .subcommand(SubCommand::with_name("stop")
            .about("Stop containers")
            .arg(Arg::with_name("container_id")
                .help("container id to stop")
                .required(false)))
        .subcommand(SubCommand::with_name("ps")
            .about("Formatted ps for running dockers"))
        .subcommand(SubCommand::with_name("psa")
            .about("Formatted ps for all dockers"))
        .subcommand(SubCommand::with_name("rc")
            .about("Remove all containers"))
        .subcommand(SubCommand::with_name("remove-none-images")
            .about("Remove none images"))
        .subcommand(SubCommand::with_name("registry")
            .about("Create a local registry"))
        .subcommand(SubCommand::with_name("ri")
            .about("Remove all images"))
        .subcommand(SubCommand::with_name("rec")
            .about("Remove exited containers"))
        .subcommand(SubCommand::with_name("kc")
            .about("Kill all containers"))
        .subcommand(SubCommand::with_name("remove-volumes")
            .about("Remove all volumes"))
        .subcommand(SubCommand::with_name("log")
            .display_order(1)
            .about("Show docker logs"))
        .subcommand(SubCommand::with_name("flog")
            .display_order(2)
            .about("Show docker logs and listen the changes"))
        .subcommand(SubCommand::with_name("elastic")
            .about("Elastic search"))
        .get_matches();

    from(matches).execute().await;
}
