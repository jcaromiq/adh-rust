use async_trait::async_trait;

use crate::commands::Command;
use crate::infra::container_repository::stop_running_containers;

pub struct KillContainers;

#[async_trait]
impl Command for KillContainers {
    async fn execute(&self) {
        stop_running_containers().await;
    }
}
