use async_trait::async_trait;
use shiplift::{Docker, ImageListOptions};
use shiplift::rep::{Image};

use crate::commands::command::Command;
use shiplift::errors::Error::Fault;

pub struct RemoveImages;

#[async_trait]
impl Command for RemoveImages {
    async fn execute(&self) {
        let docker = Docker::new();
        match docker
            .images()
            .list(&ImageListOptions::builder().all().build())
            .await {
            Ok(images) => {
                for image in images {
                    remove_image(&image).await;
                }
            }
            Err(e) => eprint!("Error {}", e)
        }
    }
}

async fn remove_image(image: &Image) {
    let docker = Docker::new();
    match docker.images().get(&image.id).delete().await {
        Ok(_) => println!("{:?} deleted! ", get_name_or_id(image)),
        Err(e) => match e {
            Fault { code, message: _ } => if code == 409 {
                eprintln!("Can not delete Image {:?} is in use", get_name_or_id(image))
            },
            _ => eprintln!("Error")
        },
    }
}
fn get_name_or_id(image: &Image) -> &String {
    match &image.repo_tags {
        None => &image.id,
        Some(n) => n.first().expect("_")
    }
}
