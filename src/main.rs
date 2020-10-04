#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use crate::ui::state::UiState;

mod config_reader;
mod ui;

fn main() {
    let app_config = config_reader::toml_parser::read_config();

    let ui_state = UiState::new(&app_config.config_tree);

    dbg!(&ui_state);

    ui::show_ui(ui_state).unwrap();
}
