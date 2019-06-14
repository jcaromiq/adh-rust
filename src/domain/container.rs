use prettytable::Table;

pub struct Containers {
    list: Vec<Container>
}

struct Container {
    id: String,
    name: String,
    image: String,
    status: String,
    ports: String,
}

impl Containers {
    pub fn print(&self) {
        let mut table = Table::new();
        table.add_row(row!["CONTAINER ID", "NAMES", "IMAGE", "STATUS", "PORTS"]);
        for c in &self.list {
            table.add_row(row![&c.id, c.name, c.image, c.status, c.ports]);
        }
        table.printstd();
    }
}

pub fn to_domain(containers: Vec<shiplift::rep::Container>) -> Containers {
    let mut container_list: Vec<Container> = Vec::new();
    for c in containers {
        let mut image = c.image.to_string();
        if image.contains("sha256:") {
            image = c.image[7..19].to_string();
        }
        let mut ports = String::from("");
        if !c.ports.is_empty() {
            ports = format!("0.0.0.0:{}->{}/{}", c.ports[0].public_port.unwrap(), c.ports[0].private_port, c.ports[0].typ);
        }
        container_list.push(Container {
            id: c.id[0..12].to_string(),
            name: c.names[0].to_string(),
            image,
            status: c.status,
            ports,
        });
    }
    return Containers {
        list: container_list
    };
}