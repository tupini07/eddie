use std::{error::Error, io};
use std::borrow::Cow::Borrowed;
use std::ops::{BitAnd, BitOr};

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction},
    Terminal,
    widgets::{Block, Borders},
};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{List, ListItem, Paragraph};

use util::event::{Event, Events};

use crate::ui::state::UiState;
use crate::ui::util::StatefulList;

mod layout;

#[allow(dead_code)]
mod util;
pub mod state;

pub fn show_ui(mut state: UiState) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        terminal.draw(|f| {
            let app_layout = layout::create_layout(f);

            let block = Block::default().borders(Borders::BOTTOM | Borders::TOP);
            let paragraph = Paragraph::new("Breadcrumbs: root > something").block(block);
            f.render_widget(paragraph, app_layout.Breadcrumbs);

            let block = Block::default().borders(Borders::ALL);
            let paragraph = Paragraph::new(Span::from(state.current_title.as_str()))
                .style(Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::White)
                    .bg(Color::DarkGray))
                .block(block);
            f.render_widget(paragraph, app_layout.Title);

            let block = Block::default().title("Group items").borders(Borders::ALL);

            let items: Vec<_> = state.current_group_items_state
                .items
                .iter()
                .map(|i| {
                    let mut lines = vec![Spans::from(*i)];
                    ListItem::new(lines).style(Style::default().fg(Color::White).bg(Color::Blue))
                }).collect();

            let lsst = List::new(items).block(block)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">> ");
            f.render_stateful_widget(lsst, app_layout.GroupContents, &mut state.current_group_items_state.state);

            let block = Block::default().title("Command outputs").borders(Borders::ALL);
            let paragraph = Paragraph::new(state.current_command_output.as_str()).block(block);
            f.render_widget(paragraph, app_layout.CommandOutput);

            let block = Block::default().title("Item description").borders(Borders::ALL);
            let paragraph = Paragraph::new(state.current_description.as_str()).block(block);
            f.render_widget(paragraph, app_layout.ItemDiscription);
        })?;

        // this Option is the Index of the selected item
        if let Some(idx) = state.current_group_items_state.state.selected() {
            // dbg!(&stateful_items.items.get(idx));
        }

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Backspace => {
                    state.current_group_items_state.unselect();
                }
                Key::Char('\t') => {
                    state.current_group_items_state.next();
                }
                Key::BackTab => {
                    state.current_group_items_state.previous();
                }
                Key::Char('\n') => {
                    // this is used to "action" on the selected item
                }
                Key::Esc => {
                    // this can be used to exit context menu like popup for input
                    // For input example see: https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
