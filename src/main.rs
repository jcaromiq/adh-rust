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
        .subcommand(SubCommand::with_name("nginx").about("Run nginx with a volume in the current directory"))
        .subcommand(SubCommand::with_name("start")
            .about("Start containers")
            .arg(Arg::with_name("options")
                .help("print debug information verbosely")
                .required(true)))
        .subcommand(SubCommand::with_name("stop")
            .about("Stop containers")
            .arg(Arg::with_name("options")
                .help("print debug information verbosely")
                .required(true)))
        .subcommand(SubCommand::with_name("ps").about("Formatted ps for running dockers"))
        .subcommand(SubCommand::with_name("psa").about("Formatted ps for all dockers"))
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
        commands::start::execute(m.value_of("options").unwrap());
    }
    if let Some(m) = matches.subcommand_matches("stop") {
        commands::stop::execute(m.value_of("options").unwrap());
    }
}