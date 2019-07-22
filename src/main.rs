#[macro_use]
extern crate prettytable;

mod commands;
mod domain;
mod infra;

use clap::{App, SubCommand, Arg};

fn main() {
    let matches = App::new("ADH")
        .version("0.1.0")
        .author("Joaco <me@joaquin-caro.es>")
        .about("docker helper")
        .subcommand(SubCommand::with_name("nginx")
            .about("Run nginx with a volume in the current directory")
            .display_order(0))
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
        .get_matches();

    if let Some(_) = matches.subcommand_matches("ps") {
        commands::ps::execute();
    }

    if let Some(_) = matches.subcommand_matches("psa") {
        commands::psa::execute();
    }

    if let Some(_) = matches.subcommand_matches("nginx") {
        commands::nginx::execute();
    }

    if let Some(m) = matches.subcommand_matches("start") {
        commands::start::execute(m.value_of("container_id").unwrap());
    }

    if let Some(m) = matches.subcommand_matches("stop") {
        commands::stop::execute(m.value_of("container_id").unwrap());
    }

    if let Some(_) = matches.subcommand_matches("rc") {
        commands::rc::execute();
    }
}