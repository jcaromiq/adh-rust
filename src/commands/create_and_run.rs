use futures::StreamExt;
use shiplift::errors::Error::Fault;
use shiplift::{ContainerListOptions, ContainerOptions, Docker, PullOptions};

pub async fn create_and_run(options: &ContainerOptions, image_name: &str) {
    let pull_options = &PullOptions::builder()
        .image(image_name)
        .build();
    create_and_start(options, pull_options).await;
}

pub async fn create_and_run_from_repo(options: &ContainerOptions,
                                      repo: &str,
                                      image_name: &str,
                                      tag: &str) {
    let pull_options = &PullOptions::builder()
        .src(repo).build();
    create_and_start(options, pull_options).await;
}

async fn create_and_start(options: &ContainerOptions, pull_options: &PullOptions) {
    let docker = Docker::new();
    let mut stream = docker
        .images()
        .pull(pull_options);
    while stream.next().await.is_some() {}
    let result = docker.containers().create(options).await;
    match result {
        Ok(result) => {
            start(&result.id).await
        }
        Err(e) => match e {
            Fault { code, message: _ } => {
                if code == 409 {
                    start_existing_container(options.name.as_ref().unwrap()).await
                } else {
                    eprintln!("Error {}", code);
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
    match docker.containers().get(id).start().await {
        Ok(()) => println!("docker {:?} created", id),
        Err(e) => eprintln!("Error: {}", e),
    }
}
