use clap::ArgMatches;

use crate::commands::create_local_registry::LocalRegistry;
use crate::commands::mysql::Mysql;
use crate::commands::nginx::Nginx;
use crate::commands::ps::Ps;
use crate::commands::psa::Psa;
use crate::commands::rc::RemoveContainers;
use crate::commands::remove_none_images::RemoveNoneImages;
use crate::commands::start::Start;
use crate::commands::stop::Stop;

pub trait Command {
    fn execute(&self);
}

pub struct Noop;

impl Command for Noop {
    fn execute(&self) { eprintln!("invalid command, try --help") }
}

pub fn from(matches: ArgMatches) -> Box<dyn Command> {
    match matches.subcommand_name() {
        Some("ps") => { Box::new(Ps) }
        Some("psa") => { Box::new(Psa) }
        Some("nginx") => { Box::new(Nginx) }
        Some("mysql") => {
            let mysql = Mysql {
                root_password: get_optional_arg(&matches, "mysql", "root_password"),
                database_name: get_optional_arg(&matches, "mysql", "database_name"),
            };
            Box::new(mysql)
        }
        Some("remove-none-images") => { Box::new(RemoveNoneImages) }
        Some("rc") => { Box::new(RemoveContainers) }
        Some("start") => {
            let container_id = get_arg(&matches, "start", "container_id");
            Box::new(Start { container_id })
        }
        Some("stop") => {
            let container_id = get_arg(&matches, "stop", "container_id");
            Box::new(Stop { container_id })
        }
        Some("clr") => { Box::new(LocalRegistry) }
        _ => { Box::new(Noop) }
    }
}

fn get_optional_arg(matches: &ArgMatches, command: &str, argument: &str) -> Option<String> {
    matches.subcommand_matches(command).unwrap().value_of(argument).map(|s| s.to_string())
}

fn get_arg(matches: &ArgMatches, command: &str, argument: &str) -> String {
    matches.subcommand_matches(command).unwrap().value_of(argument).unwrap().to_string()
}
