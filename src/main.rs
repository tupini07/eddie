mod config_reader;
mod ui;

fn main() {
    config_reader::read_config();
    ui::show_ui().unwrap();
}
