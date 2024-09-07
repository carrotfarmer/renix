use std::path;

use tabled::{Table, Tabled};

use crate::utils;

#[derive(Tabled)]
struct File {
    old_name: String,
    new_name: String,
}

impl File {
    pub fn new(old_name: String, new_name: String) -> Self {
        Self { old_name, new_name }
    }
}

pub fn get_files(path: &path::PathBuf) -> Vec<path::PathBuf> {
    let mut files = Vec::new();

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    files
}

pub fn print_table(old_paths: &Vec<path::PathBuf>, new_paths: Vec<std::path::PathBuf>) -> String {
    let mut files = Vec::new();

    for (old_path, new_path) in old_paths.iter().zip(new_paths.iter()) {
        let old_name = old_path.file_name().unwrap().to_str().unwrap().to_string();
        let new_name = new_path.file_name().unwrap().to_str().unwrap().to_string();
        files.push(File::new(old_name, new_name));
    }

    let table = Table::new(&files);
    table.to_string()
}

pub fn add_suffix_prefix(
    files: &Vec<path::PathBuf>,
    prefix: &Option<String>,
    suffix: &Option<String>,
) -> Vec<std::path::PathBuf> {
    let mut new_file_paths: Vec<std::path::PathBuf> = Vec::new();

    for file in files {
        let new_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let new_name = match &prefix {
            Some(prefix) => format!("{}{}", prefix, new_name),
            None => new_name,
        };
        let new_name = match &suffix {
            Some(suffix) => {
                let mut parts: Vec<&str> = new_name.split('.').collect();
                let ext = parts.pop().unwrap();
                let name = parts.join(".");
                format!("{}{}.{}", name, suffix, ext)
            }
            None => new_name,
        };

        let new_path = file.with_file_name(new_name);
        new_file_paths.push(new_path);
    }

    new_file_paths
}

pub fn remove_suffix_prefix(
    files: &Vec<path::PathBuf>,
    prefix: &Option<String>,
    suffix: &Option<String>,
) -> Vec<std::path::PathBuf> {
    let mut new_file_paths: Vec<std::path::PathBuf> = Vec::new();

    for file in files {
        let new_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let new_name = match &prefix {
            Some(prefix) => new_name.strip_prefix(prefix).unwrap().to_string(),
            None => new_name,
        };
        let new_name = match &suffix {
            Some(suffix) => {
                let ext = file.extension().unwrap().to_str().unwrap();
                let new_name = new_name.replace(&format!(".{}", ext), "");
                let new_name = new_name.strip_suffix(suffix);

                if new_name.is_none() {
                    utils::perror("suffix not found");
                    std::process::exit(1);
                }

                format!("{}.{}", new_name.unwrap(), ext)
            }
            None => new_name,
        };

        let new_path = file.with_file_name(new_name);
        new_file_paths.push(new_path);
        //std::fs::rename(file, new_path).unwrap();
    }

    new_file_paths
}

pub fn change_case(files: &Vec<path::PathBuf>, case: &str) -> Vec<std::path::PathBuf> {
    let mut new_file_paths: Vec<std::path::PathBuf> = Vec::new();

    for file in files {
        let new_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let new_name = match case {
            "upper" => {
                let ext = file.extension().unwrap().to_str().unwrap();
                let mut new_name = new_name.replace(&format!(".{}", ext), "");
                new_name = new_name.to_uppercase();
                format!("{}.{}", new_name, ext)
            }
            "lower" => new_name.to_lowercase(),
            _ => {
                utils::perror("invalid case");
                std::process::exit(1);
            }
        };

        let new_path = file.with_file_name(new_name);
        new_file_paths.push(new_path);
    }

    new_file_paths
}

pub fn rename_files(old_paths: Vec<std::path::PathBuf>, new_paths: Vec<std::path::PathBuf>) {
    for (old_path, new_path) in old_paths.iter().zip(new_paths.iter()) {
        std::fs::rename(old_path, new_path).unwrap();
    }
}
