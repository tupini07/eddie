use toml::Value;

use crate::config_reader::config_aggregator;

pub fn read_config() {
    let toml_dir = config_aggregator::get_proper_config_directory();
    let content = config_aggregator::get_aggregated_tomls(toml_dir);

    let value = content.parse::<Value>().unwrap();

    for a in value.as_table() {
        for k in a.keys().collect::<Vec<&String>>() {
            let item = a.get(k).unwrap();

            if let Some(_table_item) = item.as_table() {
                println!("{} its a nested table!", k);
            } else {
                println!("{} is a leaf!", k);
            }
        }
    }
}
