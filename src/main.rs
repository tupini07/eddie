mod config_reader;
mod ui;

fn main() {
    let poop = config_reader::config_aggregator::get_aggregated_tomls();
    dbg!(poop);

    ui::show_ui().unwrap();
}
