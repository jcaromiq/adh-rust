use shiplift::ContainerOptions;

use crate::commands::command::Command;
use crate::commands::create_and_run::create_and_run;

pub struct LocalRegistry;

impl Command for LocalRegistry {
    fn execute(&self) {
        //FIX: si el contenedor existe pero esta parado, no lo puede crear y tampoco lo arranca, hay que controlar el error
        // y en el caso de que el contenedor este parado, arrancarlo
        let options = &ContainerOptions::builder("registry:2")
            .name("local-registry")
            .expose(5000, "tcp", 5000).build();
        create_and_run(options);
    }
}

