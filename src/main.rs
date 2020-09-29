mod config_reader;
mod ui;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;


fn main() {
    let app_config = config_reader::toml_parser::read_config();
    dbg!(app_config);

    ui::show_ui().unwrap();
}
