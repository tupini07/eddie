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
    use std::fs::File;
    use std::io::Write;

    use rand::{Rng, thread_rng};
    use rand::distributions::Alphanumeric;

    use super::*;

    fn get_random_string(n: usize) -> String {
        let mut rng = rand::thread_rng();
        rng
            .sample_iter(&Alphanumeric)
            .take(n)
            .collect()
    }

    fn populate_tmp_files() -> (PathBuf, Vec<PathBuf>) {
        let random_prefix = get_random_string(5);

        let tmp_eddie_name = String::from("eddie-test-") + &random_prefix;

        let mut test_dir = env::temp_dir();
        test_dir.push(tmp_eddie_name);

        if !test_dir.exists() {
            create_dir(&test_dir).unwrap();
        }

        use std::fs::{File, create_dir};

        let sub_folders = vec![
            "",
            "subf",
            "subf/sub1_2",
            "subf2",
        ];

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
            "subf/sub1_2/asd.toml"
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

        return (test_dir, created_paths);
    }

    fn write_random_toml_data_to_file(path: PathBuf) {
        let mut file = File::open(path).unwrap();

        file.write_all("asdasd".as_bytes());

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