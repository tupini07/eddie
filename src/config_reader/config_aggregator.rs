use std::fs;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use std::ops::Add;

fn get_proper_config_directory() -> PathBuf {
    let home_dir_path = dirs::home_dir().unwrap();
    let home_dir = home_dir_path.to_str().unwrap();

    let mut config_dirs = vec![
        format!("{}/.config/eddie/", home_dir),
        format!("{}/.eddie/", home_dir),
        "/etc/eddie/".to_string(),
    ];

    let existing_dirs: Vec<_> = config_dirs
        .iter()
        .map(|e| Path::new(e))
        .filter(|&e| e.exists())
        .collect();

    existing_dirs
        .get(0)
        .expect("There is no usable config directories")
        .canonicalize()
        .unwrap()
}


fn get_list_of_toml_files_in_dir(ddir: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(ddir)
        .follow_links(true)
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| x.into_path())
        .filter(|x| {
            if let Some(ext) = x.extension() {
                ext == "toml"
            } else {
                false
            }
        })
        .collect()
}

fn read_file_contents(file_path: &PathBuf) -> String {
    fs::read_to_string(file_path)
        .expect(
            "Something went wrong when reading the file"
        )
}

pub fn get_aggregated_tomls() -> String {
    let ddir = get_proper_config_directory();
    let toml_files = get_list_of_toml_files_in_dir(ddir);

    toml_files
        .iter()
        .map(|e| read_file_contents(e))
        .collect::<Vec<String>>()
        .join("\n")
}

