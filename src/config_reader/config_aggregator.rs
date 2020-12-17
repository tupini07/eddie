//! The config_aggregator module is pretty simple. It exposes functionality to get
//! the appropriate configuration directory ([get_proper_config_directory]), walking
//! the directory and finding the TOML files ([get_list_of_toml_files_in_dir]), and
//! actually reading TOML files into a string ([read_file_contents]).
//!
//! The main entry point to the module is [get_aggregated_tomls] which basically just
//! reads all the config TOMLs and returns a huge string which is the aggregated
//! content of all the TOMLs.

use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

/// This method will try to "guess" which is the appropriate directory in which to
/// search for Eddie's TOML files. The method will check the following directories and
/// return the one it finds first. They're checked in the following order:
/// 1. `~/.config/eddie/`
/// 2. `~/.eddie/`
/// 1. `/etc/eddie/`
pub fn get_proper_config_directory() -> PathBuf {
    let home_dir_path = dirs::home_dir().unwrap();
    let home_dir = home_dir_path.to_str().unwrap();

    let config_dirs = vec![
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

/// This method will walk the provided [PathBuf] recursively (using [WalkDir]), and will
/// return a list of all the paths at which it found a file with a `.toml` extension.
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

/// This method will try to read the contents of the file at the specified [PathBuf] and will
/// panic if it is not able to read said file's contents.
fn read_file_contents(file_path: &PathBuf) -> String {
    fs::read_to_string(file_path).expect("Something went wrong when reading the file")
}

/// This method will read all of the toml files in Eddie's config directory (see 
/// [get_proper_config_directory]) and return a huge string which is the aggregated content
/// of all of these files.
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
    use std::fs::File;
    use std::io::Write;

    use rand::distributions::Alphanumeric;
    use rand::Rng;

    use super::*;

    fn get_random_string(n: usize) -> String {
        let rng = rand::thread_rng();
        rng.sample_iter(&Alphanumeric).take(n).collect()
    }

    fn populate_tmp_files() -> (PathBuf, Vec<PathBuf>) {
        let random_prefix = get_random_string(5);

        let tmp_eddie_name = String::from("eddie-test-") + &random_prefix;

        let mut test_dir = env::temp_dir();
        test_dir.push(tmp_eddie_name);

        if !test_dir.exists() {
            create_dir(&test_dir).unwrap();
        }

        use std::fs::{create_dir, File};

        let sub_folders = vec!["", "subf", "subf/sub1_2", "subf2"];

        for folder in sub_folders {
            let mut fd = test_dir.clone();
            fd.push(folder);

            if !fd.exists() {
                create_dir(fd).unwrap();
            }
        }

        let test_file_names = vec![
            "chom.toml",
            "poto.toml",
            "qwoto.toml",
            "subf/dfdsf.toml",
            "subf2/agwe.toml",
            "subf/sub1_2/asd.toml",
        ];

        let mut created_paths = vec![];

        for name in test_file_names {
            let mut fd = test_dir.clone();
            fd.push(name);

            if !fd.exists() {
                File::create(&fd).unwrap();
            }

            created_paths.push(fd);
        }

        (test_dir, created_paths)
    }

    fn write_random_toml_data_to_file(path: PathBuf) {
        let mut file = File::open(path).unwrap();

        file.write_all(b"asdasd");

        file.sync_all();
    }

    #[test]
    fn test_find_tomls() {
        let (tests_folder, mut expected) = populate_tmp_files();
        expected.sort();

        let mut got = get_list_of_toml_files_in_dir(tests_folder);
        got.sort();

        assert_eq!(got, expected);
    }

    #[test]
    fn test_read_appropriate_toml_data() {
        // TODO write toml contents and test that reading them returns the expected aggregated
        // content.
    }
}
