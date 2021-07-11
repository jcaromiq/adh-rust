pub struct Containers {
    pub list: Vec<Container>,
}

impl Containers {
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
}

pub fn to_domain(containers: Vec<shiplift::rep::Container>) -> Containers {
    let mut list: Vec<Container> = Vec::new();
    for container in containers {
        let id = container.id[0..12].to_string();
        let name = container.names[0].to_string();
        let mut image = container.image.to_string();
        let status = container.status;
        if image.contains("sha256:") {
            image = container.image[7..19].to_string();
        }
        let mut public_ports = String::new();
        let mut private_ports = String::new();
        if !container.ports.is_empty() {
            for port in container.ports {
                if port.public_port.is_some() {
                    public_ports.push_str(&format!(
                        "0.0.0.0:{}->{}\n",
                        port.public_port.unwrap(),
                        port.private_port,
                    ));
                } else {
                    private_ports.push_str(&format!("{} ", port.private_port))
                }
            }
        }
        let ports = public_ports + &private_ports;
        list.push(Container {
            id,
            name,
            image,
            status,
            ports,
        })
    }
    Containers { list }
}
