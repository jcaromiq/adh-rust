use async_trait::async_trait;

use crate::commands::Command;
use crate::infra::container_repository::get_all_containers;
use crate::infra::printer;

pub struct Psa;

#[async_trait]
impl Command for Psa {
    async fn execute(&self) {
        printer::print(get_all_containers().await);
    }
}
