use futures::Future;
use shiplift::{Docker, ImageFilter, ImageListOptions};

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
            .and_then(move |images|
                {
                    //TODO: move to fun
                    for image in images {
                        let ff = docker.images()
                            .get(image.id.as_str())
                            .delete()
                            .map(move |_| println!("deleted {}", image.id))
                            .map_err(|e| eprintln!("Error: {} deleting container", e));
                        tokio::spawn(ff);
                    }
                    Ok(())
                })
            .map(|_| println!("All none images deleted"))
            .map_err(|e| eprintln!("Error: {}", e));
        tokio::run(op);
    }
}
