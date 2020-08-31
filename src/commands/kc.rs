use async_trait::async_trait;

use crate::commands::command::Command;
use crate::infra::repository::stop_running_containers;

pub struct KillContainers;

#[async_trait]
impl Command for KillContainers {
    async fn execute(&self) {
        stop_running_containers().await;
    }
}
