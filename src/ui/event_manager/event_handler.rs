use std::fmt::format;
use std::process::{Command, Stdio};

use crossterm::event::KeyCode;

use crate::ui::util::event::Event;
use crate::{config_reader::config_structs::ConfigNode, ui::state::UiState};

fn execute_command(command_node: &ConfigNode) -> String {
    let mut command = Command::new(if command_node.opens_external {
        &crate::APP_CONFIG.eddie_config.terminal_emulator
    } else {
        &crate::APP_CONFIG.eddie_config.shell
    });

    if command_node.opens_external {
        command
            .arg(&crate::APP_CONFIG.eddie_config.terminal_emulator_command_arg)
            .arg(&crate::APP_CONFIG.eddie_config.shell)
            .arg("-c")
            .arg(&command_node.command)
    } else {
        command.arg("-c").arg(&command_node.command)
    };

    let child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!(
            "failed to execute process. Command: {:?}",
            command
        ));

    // if the process opens an external terminal then don't wait for output
    if command_node.opens_external {
        return String::from("Executing command in external terminal...");
    } else {
        match child.wait_with_output() {
            Ok(o) => String::from_utf8(o.stdout).unwrap(),
            Err(_) => "There was an error while decoding the command's output!".to_string(),
        }
    }
}

pub fn handle_event(ev: Event<KeyCode>, state: &mut UiState) -> Option<bool> {
    match ev {
        Event::Input(input) => match input {
            KeyCode::Char('q') => {
                return Some(true);
            }
            KeyCode::Backspace => {
                // state.current_group_items_state.unselect();
                state.exit_current_node();
            }
            KeyCode::Tab => {
                state.group_items_state.next();
                state.update_description();
            }
            KeyCode::BackTab => {
                state.group_items_state.previous();
                state.update_description();
            }
            KeyCode::Enter => {
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
            KeyCode::Esc => {
                // this can be used to exit context menu like popup for input
                // For input example see: https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs
            }
            _ => {}
        },
        Event::Tick => {}
    };

    Some(false)
}
