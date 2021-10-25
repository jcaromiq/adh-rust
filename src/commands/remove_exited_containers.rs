use async_trait::async_trait;

use crate::commands::Command;
use crate::infra::container_repository::{delete, get_exited_containers};

pub struct RemoveExitedContainers;

#[async_trait]
impl Command for RemoveExitedContainers {
    async fn execute(&self) {
        let containers = get_exited_containers().await;
        delete(containers).await;
    }
}
