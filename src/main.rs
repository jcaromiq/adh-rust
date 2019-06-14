#[macro_use]
extern crate prettytable;

mod commands;
mod domain;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("ApiumDockerHelper")
        .version("0.1.0")
        .author("Joaco <me@joaquin-caro.es>")
        .about("adh")
        .subcommand(SubCommand::with_name("ps").about("Formatted ps for running dockers"))
        .subcommand(SubCommand::with_name("psa").about("Formatted ps for all dockers"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("ps") {
        commands::ps::execute();
    }
    if let Some(_) = matches.subcommand_matches("psa") {
        commands::psa::execute();
    }
}