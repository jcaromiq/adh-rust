use shiplift::{Docker, Error, ImageFilter, ImageListOptions};
use shiplift::rep::Image;
use tokio::prelude::Future;
use futures::StreamExt;

use crate::commands::command::Command;

pub struct RemoveNoneImages;

impl Command for RemoveNoneImages {
    fn execute(&self) {
        let docker = Docker::new();

        let dangling_filter = ImageListOptions::builder()
            .filter(vec!(ImageFilter::Dangling))
            .build();

        let op = docker
            .images()
            .list(&dangling_filter)
            .and_then(delete)
            .map(|_| ())
            .map_err(|e| eprintln!("Error: {}", e));
        tokio::run(op);
    }
}

fn delete(images: Vec<Image>) -> std::result::Result<(), Error> {
    let docker = Docker::new();
    for image in images {
        let ff = docker.images()
            .get(image.id.as_str())
            .delete()
            .map(move |_| println!("deleted {}", image.id))
            .map_err(|e| eprintln!("Error: {} deleting container", e));
        tokio::spawn(ff);
    }
    Ok(())
}
