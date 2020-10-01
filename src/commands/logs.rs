use async_trait::async_trait;
use futures::StreamExt;
use shiplift::{Docker, LogsOptions};
use shiplift::tty::TtyChunk;

use crate::commands::command::Command;
use crate::infra::container_repository::get_all_containers;
use crate::infra::container_selector::select_container;

pub struct Logs {
    pub(crate) follow: bool,
}

#[async_trait]
impl Command for Logs {
    async fn execute(&self) {
        let containers = get_all_containers().await;
        if containers.is_empty() {
            println!("No containers found");
            return;
        }
        match select_container(containers) {
            None => { println!("No containers found"); }
            Some(selected) => {
                let docker = Docker::new();
                let mut logs_stream = docker.containers().get(&selected).logs(
                    &LogsOptions::builder()
                        .follow(self.follow)
                        .stdout(true)
                        .stderr(true)
                        .build(),
                );

                while let Some(log_result) = logs_stream.next().await {
                    match log_result {
                        Ok(chunk) => print_chunk(chunk),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
        }
    }
}

fn print_chunk(chunk: TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => println!("{}", std::str::from_utf8(&bytes).unwrap()),
        TtyChunk::StdErr(bytes) => println!("{}", std::str::from_utf8(&bytes).unwrap()),
        TtyChunk::StdIn(_) => unreachable!(),
    }
}

impl Logs {}
