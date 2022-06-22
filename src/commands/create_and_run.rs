use futures::StreamExt;
use shiplift::errors::Error::Fault;
use shiplift::{ContainerListOptions, ContainerOptions, Docker, PullOptions};

pub async fn create_and_run_latest(options: &ContainerOptions, image_name: &str) {
    let pull_options = &PullOptions::builder().image(image_name).build();
    create_and_start(options, pull_options).await;
}

pub async fn create_and_run_with_tag(options: &ContainerOptions, image_name: &str, tag: &str) {
    let pull_options = &PullOptions::builder().image(image_name).tag(tag).build();
    create_and_start(options, pull_options).await;
}

async fn create_and_start(options: &ContainerOptions, pull_options: &PullOptions) {
    let docker = Docker::new();
    let mut stream = docker.images().pull(pull_options);
    while let Some(pull_result) = stream.next().await {
        match pull_result {
            Ok(output) => println!("{:?}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    let result = docker.containers().create(options).await;
    match result {
        Ok(result) => start(&result.id).await,
        Err(e) => match e {
            Fault { code, message } => {
                if code == 409 {
                    start_existing_container(options.name.as_ref().unwrap()).await
                } else {
                    eprintln!("Error with code {} and message {}", code, message);
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
            let container = containers
                .iter()
                .find(|c| c.names.contains(&format!("/{}", name)));

            start(&container.unwrap().id).await;
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
