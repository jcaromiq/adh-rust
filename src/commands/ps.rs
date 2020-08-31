use async_trait::async_trait;

use crate::commands::command::Command;
use crate::infra::printer;
use crate::infra::repository::get_running_containers;

pub struct Ps;

#[async_trait]
impl Command for Ps {
    async fn execute(&self) {
        printer::print(get_running_containers().await);
    }
}
