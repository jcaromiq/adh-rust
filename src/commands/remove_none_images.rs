use futures::Future;
use shiplift::{Docker, Error, ImageFilter, ImageListOptions};
use shiplift::rep::Image;

pub fn execute() {
    let docker = Docker::new();

    let dangling_filter = ImageListOptions::builder()
        .filter(vec!(ImageFilter::Dangling))
        .build();

    let op = docker
        .images()
        .list(&dangling_filter)
        .and_then(move |images| {
            delete(images, docker)
        })
        .map(|_| println!("All none images deleted"))
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(op);
}

fn delete(images: Vec<Image>, docker: Docker) -> std::result::Result<(), Error> {
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