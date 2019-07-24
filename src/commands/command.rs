use clap::ArgMatches;

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

pub fn factory(matches: ArgMatches) -> Box<dyn Command> {
    if let Some(_) = matches.subcommand_matches("ps") {
        return Box::new(Ps);
    }

    if let Some(_) = matches.subcommand_matches("psa") {
        return Box::new(Psa);
    }

    if let Some(_) = matches.subcommand_matches("nginx") {
        return Box::new(Nginx);
    }

    if let Some(_) = matches.subcommand_matches("remove-none-images") {
        return Box::new(RemoveNoneImages);
    }

    if let Some(_) = matches.subcommand_matches("rc") {
        return Box::new(RemoveContainers);
    }

    if let Some(m) = matches.subcommand_matches("start") {
        let container_id = m.value_of("container_id").unwrap().to_string();
        return Box::new(Start { container_id });
    }

    if let Some(m) = matches.subcommand_matches("stop") {
        let container_id = m.value_of("container_id").unwrap().to_string();
        return Box::new(Stop { container_id });
    }

    return Box::new(Psa);
}