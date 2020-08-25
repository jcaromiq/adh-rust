extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use shiplift::ContainerOptions;

use crate::commands::command::Command;
use crate::commands::create_and_run::create_and_run;

pub struct Mysql {
    pub root_password: Option<String>,
    pub database_name: Option<String>,
}

use async_trait::async_trait;

#[async_trait]
impl Command for Mysql {
    async fn execute(&self) {
        let mut env = Vec::new();
        let password = match &self.root_password {
            None => thread_rng().sample_iter(&Alphanumeric).take(6).collect(),
            Some(n) => n.clone(),
        };

        env.push(format!("MYSQL_ROOT_PASSWORD={}", password));

        if let Some(n) = &self.database_name {
            env.push(format!("MYSQL_DATABASE={}", n));
        }
        let options = &ContainerOptions::builder("mysql")
            .name("adh-mysql")
            .env(env.iter().map(|s| &**s).collect())
            .expose(3306, "tcp", 3306)
            .build();

        create_and_run(options, "mysql:latest").await;
        println!("mysql root password: {}", password);
    }
}
