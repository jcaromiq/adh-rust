use async_trait::async_trait;

use crate::commands::command::Command;
use crate::infra::repository::{get_all_containers, delete};

pub struct RemoveContainers;

#[async_trait]
impl Command for RemoveContainers {
    async fn execute(&self) {
        let containers = get_all_containers().await;
        delete(containers).await;
    }
}

