use async_trait::async_trait;
use shiplift::volume::VolumeInfo;
use shiplift::Docker;

use crate::commands::Command;

pub struct RemoveVolumes;

#[async_trait]
impl Command for RemoveVolumes {
    async fn execute(&self) {
        let docker = Docker::new();
        match docker.volumes().list().await {
            Ok(volumes) => delete(volumes).await,
            Err(e) => eprintln!("Error getting volumes {}", e),
        }
    }
}

async fn delete(volumes: Vec<VolumeInfo>) {
    let docker = Docker::new();
    for v in volumes {
        match docker.volumes().get(&v.name).delete().await {
            Ok(_) => println!("{} deleted!", v.name),
            Err(e) => eprintln!("Error deleting volume {} {}", v.name, e),
        }
    }
}
