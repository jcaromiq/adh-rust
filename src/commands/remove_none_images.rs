use async_trait::async_trait;
use shiplift::rep::Image;
use shiplift::{Docker, ImageFilter, ImageListOptions};

use crate::commands::Command;

pub struct RemoveNoneImages;

#[async_trait]
impl Command for RemoveNoneImages {
    async fn execute(&self) {
        let docker = Docker::new();

        let dangling_filter = ImageListOptions::builder()
            .filter(vec![ImageFilter::Dangling])
            .build();

        match docker.images().list(&dangling_filter).await {
            Ok(images) => delete(images).await,
            Err(e) => eprintln!("Error: {}", e),
        };
    }
}

async fn delete(images: Vec<Image>) {
    let docker = Docker::new();
    for image in images {
        match docker.images().get(image.id.as_str()).delete().await {
            Ok(_) => println!("deleted {}", image.id),
            Err(e) => eprintln!("Error: {} deleting container", e),
        };
    }
}
