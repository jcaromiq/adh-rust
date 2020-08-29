#[macro_use]
extern crate prettytable;

use clap::{App, Arg, SubCommand};

use crate::commands::command::from;

mod commands;
mod domain;
mod infra;

#[tokio::main]
async fn main() {
    let matches = App::new("ADH")
        .version("0.1.3")
        .author("Joaco <me@joaquin-caro.es>")
        .about("docker helper")
        .subcommand(SubCommand::with_name("nginx")
            .about("Run nginx with a volume in the current directory")
            .display_order(0))
        .subcommand(SubCommand::with_name("mysql")
            .about("Run mysql")
            .display_order(0)
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
            .display_order(1)
            .arg(Arg::with_name("container_id")
                .help("container id to start")
                .required(true)))
        .subcommand(SubCommand::with_name("stop")
            .about("Stop containers")
            .display_order(2)
            .arg(Arg::with_name("container_id")
                .help("container id to stop")
                .required(true)))
        .subcommand(SubCommand::with_name("ps")
            .about("Formatted ps for running dockers")
            .display_order(3))
        .subcommand(SubCommand::with_name("psa")
            .display_order(4)
            .about("Formatted ps for all dockers"))
        .subcommand(SubCommand::with_name("rc")
            .display_order(5)
            .about("Remove all containers"))
        .subcommand(SubCommand::with_name("remove-none-images")
            .display_order(6)
            .about("Remove none images"))
        .subcommand(SubCommand::with_name("clr")
            .display_order(7)
            .about("Create a local registry"))
        .subcommand(SubCommand::with_name("ri")
            .display_order(8)
            .about("Remove all images"))
        .get_matches();

    from(matches).execute().await;
}
