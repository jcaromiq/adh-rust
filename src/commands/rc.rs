use async_trait::async_trait;

use crate::commands::Command;
use crate::infra::container_repository::{delete, get_all_containers};

pub struct RemoveContainers;

#[async_trait]
impl Command for RemoveContainers {
    async fn execute(&self) {
        let containers = get_all_containers().await;
        delete(containers).await;
    }
}
