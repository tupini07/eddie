mod config_reader;
mod ui;

fn main() {
    config_reader::toml_parser::read_config();

    ui::show_ui().unwrap();
}
