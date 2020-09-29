use async_trait::async_trait;

use crate::commands::command::Command;
use crate::infra::container_repository::get_running_containers;
use crate::infra::printer;

pub struct Ps;

#[async_trait]
impl Command for Ps {
    async fn execute(&self) {
        printer::print(get_running_containers().await);
    }
}
