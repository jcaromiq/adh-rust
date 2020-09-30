use async_trait::async_trait;
use futures::StreamExt;
use shiplift::builder::LogsOptionsBuilder;
use shiplift::tty::TtyChunk;
use shiplift::{Docker, LogsOptions};

use crate::commands::command::Command;
use crate::infra::container_repository::get_all_containers;
use crate::infra::container_selector::select_container;

pub struct Logs;

#[async_trait]
impl Command for Logs {
    async fn execute(&self) {
        let containers = get_all_containers().await;
        let selected = select_container(containers);
        let docker = Docker::new();
        let mut logs_stream = docker.containers().get(&selected).logs(
            &LogsOptions::builder()
                .follow(true)
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

fn print_chunk(chunk: TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => println!("{}", std::str::from_utf8(&bytes).unwrap()),
        TtyChunk::StdErr(bytes) => println!("{}", std::str::from_utf8(&bytes).unwrap()),
        TtyChunk::StdIn(_) => unreachable!(),
    }
}

impl Logs {}
