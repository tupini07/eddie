mod config_reader;
mod ui;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;


fn main() {
    config_reader::toml_parser::read_config();

    ui::show_ui().unwrap();
}
