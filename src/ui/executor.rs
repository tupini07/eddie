use std::process::{Command, Stdio};

use crate::config_reader::config_structs::{ConfigNode, EddieConfig};

pub fn execute_command(eddie_config: &EddieConfig, command_node: &ConfigNode) -> String {
    let mut command = Command::new(if command_node.opens_external {
        &eddie_config.terminal_emulator
    } else {
        &eddie_config.shell
    });

    if command_node.opens_external {
        command
            .arg(&eddie_config.terminal_emulator_command_arg)
            .arg(&eddie_config.shell)
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
