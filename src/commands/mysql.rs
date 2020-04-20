use shiplift::ContainerOptions;

use crate::commands::command::Command;
use crate::commands::create_and_run::create_and_run;

pub struct Mysql {
    pub root_password: String,
    pub database_name: Option<String>,
}

impl Mysql {
    pub fn get_root_password(&self) -> String {
        format!("MYSQL_ROOT_PASSWORD={}", self.root_password)
    }

    pub fn get_database_name(&self) -> String {
        self.database_name.as_ref()
            .map(|n| format!("MYSQL_DATABASE={}", n))
            .unwrap_or_else(|| String::from(""))
    }
}

impl Command for Mysql {
    fn execute(&self) {
        let mut env = Vec::new();
        env.push(self.get_root_password());
        if !self.get_database_name().is_empty() {
            env.push(self.get_database_name());
        }

        let options = &ContainerOptions::builder("mysql")
            .name("adh-mysql")
            .env(env.iter().map(|s| &**s).collect())
            .expose(3306, "tcp", 3306).build();

        create_and_run(options, "mysql:latest");
    }
}
