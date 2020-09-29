
use async_trait::async_trait;

use crate::commands::command::Command;
use crate::infra::container_repository::get_all_containers;
use crate::infra::container_selector::select_container;

pub struct Logs;

#[async_trait]
impl Command for Logs {
    async fn execute(&self) {
        let containers = get_all_containers().await;
        let selected = select_container(containers);

        println!("Selected {} ", selected);
    }
}

impl Logs {}
