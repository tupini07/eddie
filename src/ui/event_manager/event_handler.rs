use std::process::{Command, Stdio};

use termion::event::Key;

use crate::ui::util::event::Event;
use crate::{config_reader::config_structs::ConfigNode, ui::state::UiState};

fn execute_command(command_node: &ConfigNode) -> String {
    let mut command = Command::new(if command_node.opens_external {
        "alacritty"
    } else {
        "fish"
    });

    if command_node.opens_external {
        command
            .arg("--command")
            .arg("fish")
            .arg("-c")
            .arg(&command_node.command)
    } else {
        command.arg("-c").arg(&command_node.command)
    };

    let child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    match child.wait_with_output() {
        Ok(o) => String::from_utf8(o.stdout).unwrap(),
        Err(_) => "There was an error while decoding the command's output!".to_string(),
    }
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
                    state.command_output = "".to_string();

                    state.command_output = execute_command(&selected_node);

                    // always triggered a forced redraw after a command is executed
                    state.need_redraw = true;
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
