use termion::event::Key;

use crate::ui::state::UiState;
use crate::ui::util::event::Event;

pub fn handle_event(ev: Event<Key>, state: &mut UiState) -> bool {
    match ev {
        Event::Input(input) => match input {
            Key::Char('q') => {
                return true;
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
    };

    return false;
}