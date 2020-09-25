use std::fs;
use std::fs::read_dir;
use std::ops::Add;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn get_proper_config_directory() -> PathBuf {
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

pub fn get_aggregated_tomls(ddir: PathBuf) -> String {
    let toml_files = get_list_of_toml_files_in_dir(ddir);

    toml_files
        .iter()
        .map(|e| read_file_contents(e))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    fn populate_tmp_files() -> Vec<PathBuf> {
        use std::fs::{File, create_dir};

        let folders = vec![
            "eddie-test",
            "eddie-test/subf",
            "eddie-test/subf2",
            "eddie-test/subf/sub1_2"
        ];

        for folder in folders {
            let mut dir = env::temp_dir();
            dir.push(folder);

            if !dir.exists() {
                create_dir(dir).unwrap();
            }
        }

        let names = vec![
            "eddie-test/chom.toml",
            "eddie-test/poto.toml",
            "eddie-test/qwoto.toml",
            "eddie-test/subf/dfdsf.toml",
            "eddie-test/subf2/agwe.toml",
            "eddie-test/subf/sub1_2/asd.toml"
        ];

        let mut created_paths = vec![];

        for name in names {
            let mut dir = env::temp_dir();
            dir.push(name);

            if !dir.exists() {
                File::create(&dir).unwrap();
            }

            created_paths.push(dir);
        }

        return created_paths;
    }

    #[test]
    fn test_find_tomls() {
        let mut expected = populate_tmp_files();
        expected.sort();

        let mut dir = env::temp_dir();
        dir.push("eddie-test");

        let mut got = get_list_of_toml_files_in_dir(dir);
        got.sort();

        assert_eq!(got, expected);
    }
}