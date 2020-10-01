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
    for c in containers {
        let id = c.id[0..12].to_string();
        let name = c.names[0].to_string();
        let mut image = c.image.to_string();
        let status = c.status;
        if image.contains("sha256:") {
            image = c.image[7..19].to_string();
        }
        let mut ports = "".to_string();
        if !c.ports.is_empty() {
            //FIX: iterate over port
            if c.ports[0].public_port.is_some() {
                ports = format!(
                    "0.0.0.0:{}->{}/{}",
                    c.ports[0].public_port.unwrap(),
                    c.ports[0].private_port,
                    c.ports[0].typ
                );
            } else {
                ports = format!("{}/{}", c.ports[0].private_port, c.ports[0].typ);
            }
        }
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
