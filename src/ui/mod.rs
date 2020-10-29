use std::{error::Error, io};

use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

use util::event::Events;

use crate::ui::state::UiState;

mod drawer;
mod event_manager;
mod layout;
pub mod state;
#[allow(dead_code)]
mod util;

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

            drawer::draw_tui(frame, &app_layout, &mut state);
        })?;

        // force redraw if necessary
        if state.need_redraw {
            terminal.resize(terminal.size().unwrap());
            state.need_redraw = false;
        }

        // this Option is the Index of the selected item
        if let Some(_idx) = state.group_items_state.state.selected() {
            // dbg!(&stateful_items.items.get(idx));
        }

        let ev = events.next()?;
        match event_manager::event_handler::handle_event(ev, &mut state) {
            None => break,
            Some(v) => {
                if v {
                    break;
                }
            }
        }
    }

    Ok(())
}
