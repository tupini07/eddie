//! Welcome to [Eddie](https://hitchhikers.fandom.com/wiki/Eddie_the_Computer).

use config_reader::config_structs::AppConfig;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod config_reader;
mod ui;

fn main() {
    let app_config: AppConfig = config_reader::toml_parser::read_config();

    ui::show_ui(app_config);
}
