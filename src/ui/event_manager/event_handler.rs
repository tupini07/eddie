use std::process::Command;

use termion::event::Key;

use crate::ui::state::UiState;
use crate::ui::util::event::Event;

fn execute_command(command: &str) -> String {
    let x = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    String::from_utf8(x.stdout)
        .unwrap_or("Couldn't decode command output".to_string())
}

pub fn handle_event(ev: Event<Key>, state: &mut UiState) -> Option<bool> {
    match ev {
        Event::Input(input) => match input {
            Key::Char('q') => {
                return Some(true);
            }
            Key::Backspace => {
                // state.current_group_items_state.unselect();
                state.exit_current_node();
            }
            Key::Char('\t') => {
                state.group_items_state.next();
                state.update_description();
            }
            Key::BackTab => {
                state.group_items_state.previous();
                state.update_description();
            }
            Key::Char('\n') => {
                let selected_node = state.get_selected_node()?;
                if selected_node.is_leaf() {
                    state.command_output = execute_command(&selected_node.command);
                } else {
                    // this is used to "action" on the selected item
                    state.enter_selected_node();
                }
            }
            Key::Esc => {
                // this can be used to exit context menu like popup for input
                // For input example see: https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs
            }
            _ => {}
        },
        Event::Tick => {}
    };

    Some(false)
}