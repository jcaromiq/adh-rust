use prettytable::{Table, format};

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
        let format = format::FormatBuilder::new()
            .borders('|')
            .separators(&[format::LinePosition::Top],
                        format::LineSeparator::new('─', ' ', '┌', '┐'))
            .separators(&[format::LinePosition::Bottom],
                        format::LineSeparator::new('─', ' ', '└', '┘'))
            .padding(4, 4)
            .build();
        table.set_format(format);
        table.set_titles(row![bc=> "CONTAINER ID", "NAMES", "IMAGE", "STATUS", "PORTS"]);
        for c in &self.list {
            table.add_row(row![c.id, c.name, c.image, c.status, c.ports]);
        }
        table.printstd();
    }
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
                ports = format!("0.0.0.0:{}->{}/{}", c.ports[0].public_port.unwrap(), c.ports[0].private_port, c.ports[0].typ);
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
    return Containers { list };
}