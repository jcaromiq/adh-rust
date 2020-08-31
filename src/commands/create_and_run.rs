use futures::StreamExt;
use shiplift::errors::Error::Fault;
use shiplift::{ContainerListOptions, ContainerOptions, Docker, PullOptions};

pub async fn create_and_run(options: &ContainerOptions, image_name: &str) {
    let docker = Docker::new();
    let mut stream = docker
        .images()
        .pull(&PullOptions::builder().image(image_name).build());
    while stream.next().await.is_some() {}

    let result = docker.containers().create(options).await;
    match result {
        Ok(result) => start(&result.id).await,
        Err(e) => match e {
            Fault { code, message: _ } => {
                if code == 409 {
                    start_existing_container(options.name.as_ref().unwrap()).await
                }
            }
            _ => eprintln!("Error"),
        },
    };
}

async fn start_existing_container(name: &str) {
    let docker = Docker::new();

    match docker
        .containers()
        .list(&ContainerListOptions::builder().all().build())
        .await
    {
        Ok(containers) => {
            let nginx = containers
                .iter()
                .find(|c| c.names.contains(&format!("/{}", name)));

            start(&nginx.unwrap().id).await;
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

async fn start(id: &str) {
    let docker = Docker::new();
    match docker.containers().get(&id).start().await {
        Ok(()) => println!("docker {:?} created", id),
        Err(e) => eprintln!("Error: {}", e),
    }
}
