use futures::StreamExt;
use shiplift::{ContainerOptions, Docker, PullOptions};

pub async fn create_and_run(options: &ContainerOptions, image_name: &str) {
    //TODO: container can exists but maybe is stoped, actually can not create and also can not started
    // need handle this case and start it
    let docker = Docker::new();
    let mut stream = docker
        .images()
        .pull(&PullOptions::builder().image(image_name).build());
    while stream.next().await.is_some() {}

    let result = docker.containers().create(options).await;
    match result {
        Ok(result) => start(&result.id).await,
        Err(e) => eprintln!("Error: {}", e),
    };
}

async fn start(id: &str) {
    let docker = Docker::new();
    match docker.containers().get(&id).start().await {
        Ok(()) => println!("docker {:?} created", id),
        Err(e) => eprintln!("Error: {}", e),
    }
}
