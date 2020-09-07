use async_trait::async_trait;

use crate::commands::command::Command;
use crate::infra::printer;
use crate::infra::container_repository::get_all_containers;

pub struct Psa;

#[async_trait]
impl Command for Psa {
    async fn execute(&self) {
        printer::print(get_all_containers().await);
    }
}
