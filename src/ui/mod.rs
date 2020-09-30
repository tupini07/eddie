use std::{error::Error, io};

use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction},
    Terminal,
    widgets::{Block, Borders},
};

use util::event::Events;

use crate::ui::state::UiState;

#[allow(dead_code)]
mod util;
pub mod state;
mod layout;
mod event_manager;
mod drawer;

pub fn show_ui(mut state: UiState) -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        terminal.draw(|frame| {
            let app_layout = layout::create_layout(frame);

            drawer::draw_tui(
                frame,
                &app_layout,
                &mut state,
            );
        })?;

        // this Option is the Index of the selected item
        if let Some(_idx) = state.current_group_items_state.state.selected() {
            // dbg!(&stateful_items.items.get(idx));
        }

        let ev = events.next()?;
        if event_manager::event_handler::handle_event(ev, &mut state) {
            break;
        }
    }

    Ok(())
}
