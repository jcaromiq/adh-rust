use std::io;

use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem};
use tui::Terminal;

use crate::domain::container::Containers;
use crate::utils::events::{Event, Events};
use crate::utils::StatefulList;

pub fn select_container(containers: Containers) -> Option<String> {
    if containers.is_empty() {
        return None;
    }
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut container_list = StatefulList::with_items(containers.list);

    let key_events = Events::new();
    let mut selected: Option<String> = None;
    loop {
        terminal.draw(|f| {
            let items: Vec<ListItem> = container_list
                .items
                .iter()
                .map(|it| ListItem::new(format!("[{}] {}", it.id, it.name)))
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(
                    "Select container with arrow keys and Enter to confirm (press 'q' to exit)",
                ))
                .highlight_style(
                    Style::default()
                        .bg(Color::Gray)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );

            f.render_stateful_widget(list, f.size(), &mut container_list.state);
        });

        if let Event::Input(input) = key_events.next().unwrap() {
            match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    container_list.first();
                }
                Key::Right => {
                    container_list.last();
                }
                Key::Down => {
                    container_list.next();
                }
                Key::Up => {
                    container_list.previous();
                }
                Key::Char('\n') => match container_list.state.selected() {
                    None => {}
                    Some(index) => {
                        let a = &container_list.items.get(index).unwrap().id;
                        selected = Some(a.to_string());
                        break;
                    }
                },
                _ => {}
            }
        }
    }
    selected
}
