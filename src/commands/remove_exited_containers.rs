use async_trait::async_trait;

use crate::commands::command::Command;
use crate::infra::repository::{get_exited_containers, delete};

pub struct RemoveExitedContainers;

#[async_trait]
impl Command for RemoveExitedContainers {
    async fn execute(&self) {
        let containers = get_exited_containers().await;
        delete(containers).await;
    }
}