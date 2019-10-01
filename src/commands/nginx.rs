use shiplift::ContainerOptions;

use crate::commands::command::Command;
use crate::commands::create_and_run::create_and_run;

pub struct Nginx;

impl Command for Nginx {
    fn execute(&self) {
        //FIX: si el contenedor existe pero esta parado, no lo puede crear y tampoco lo arranca, hay que controlar el error
        // y en el caso de que el contenedor este parado, arrancarlo
        let options = &ContainerOptions::builder("nginx")
            .name("adh-nginx")
            .expose(80, "tcp", 8888).build();
        create_and_run(options);
    }
}
