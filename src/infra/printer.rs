use prettytable::{format, Table};

use crate::domain::container::Containers;

pub fn print(containers: Containers) {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .borders('|')
        .separators(
            &[format::LinePosition::Top],
            format::LineSeparator::new('─', ' ', '┌', '┐'),
        )
        .separators(
            &[format::LinePosition::Bottom],
            format::LineSeparator::new('─', ' ', '└', '┘'),
        )
        .padding(4, 4)
        .build();
    table.set_format(format);
    table.add_row(row![bc=> "CONTAINER ID", "NAMES", "IMAGE", "STATUS", "PORTS"]);
    for c in &containers.list {
        table.add_row(row![c.id, c.name, c.image, c.status, c.ports]);
    }
    table.printstd();
}
