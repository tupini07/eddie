//! Welcome to [Eddie](https://hitchhikers.fandom.com/wiki/Eddie_the_Computer).

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate lazy_static;

use config_reader::config_structs::AppConfig;

use crate::ui::state::UiState;

mod config_reader;
mod ui;

lazy_static! {
    static ref APP_CONFIG: AppConfig = config_reader::toml_parser::read_config();
}

fn main() {
    let ui_state = UiState::new(&APP_CONFIG.config_tree);

    ui::show_ui(ui_state).unwrap();
}
