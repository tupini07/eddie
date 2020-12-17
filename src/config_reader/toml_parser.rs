use toml::Value;

use crate::config_reader::config_aggregator;
use crate::config_reader::config_structs::*;

fn get_sub_table_keys(val: &Value) -> Option<Vec<&String>> {
    let table = val.as_table()?;
    let keys = table.keys().collect::<Vec<_>>();

    let mut table_keys = Vec::new();

    for key in keys {
        let item = table.get(key)?;
        if let Some(_i) = item.as_table() {
            table_keys.push(key);
        }
    }

    if !table_keys.is_empty() {
        Some(table_keys)
    } else {
        None
    }
}

fn parse_nodes(val: &Value) -> ConfigNode {
    let table = val.as_table().unwrap();

    let name = table.get("name").unwrap().as_str().unwrap();
    let description = match table.get("description") {
        None => "NO DESCRIPTION PROVIDED",
        Some(e) => e.as_str().unwrap(),
    };

    let command = if let Some(command) = table.get("command") {
        command.as_str().unwrap()
    } else {
        ""
    };

    let opens_external = if let Some(opens_external) = table.get("external") {
        opens_external.as_bool().unwrap()
    } else {
        false
    };

    let sub_tables = get_sub_table_keys(val);
    let parsed_subtables: Vec<ConfigNode> = sub_tables
        .unwrap_or_default()
        .iter()
        .map(|&e| parse_nodes(table.get(e).unwrap()))
        .collect();

    ConfigNode {
        name: name.to_string(),
        description: description.to_string(),
        command: command.to_string(),
        opens_external: opens_external,
        children: if parsed_subtables.is_empty() {
            None
        } else {
            Some(parsed_subtables)
        },
    }
}

pub fn read_config() -> AppConfig {
    let toml_dir = config_aggregator::get_proper_config_directory();
    let content = config_aggregator::get_aggregated_tomls(toml_dir);

    let value = content.parse::<Value>().unwrap();
    let root_table = value.as_table().unwrap();

    let to_skip = vec!["ship"];

    let top_level_children: Vec<ConfigNode> = root_table
        .keys()
        .into_iter()
        .filter(|&e| !to_skip.contains(&e.as_str()))
        .filter_map(|e| root_table.get(e))
        .map(parse_nodes)
        .collect();

    AppConfig {
        eddie_config: EddieConfig {},
        config_tree: ConfigNode {
            name: "Root config node".to_string(),
            description: "This is the root node of the configuration tree".to_string(),
            command: "".to_string(),
            opens_external: false,
            children: if !top_level_children.is_empty() {
                Some(top_level_children)
            } else {
                None
            },
        },
    }
}
