use std::path;

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct File {
    name: String,
}

impl File {
    pub fn new(name: String) -> Self {
        Self { name }
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

pub fn print_files(files: Vec<path::PathBuf>) -> String {
    let files: Vec<File> = files
        .iter()
        .map(|file| File::new(file.to_str().unwrap().to_string()))
        .collect();

    let table = Table::new(&files).to_string();
    return table;
}

pub fn add_suffix_prefix(
    files: &Vec<path::PathBuf>,
    prefix: &Option<String>,
    suffix: &Option<String>,
) {
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
        std::fs::rename(file, new_path).unwrap();
    }
}

pub fn remove_suffix_prefix(
    files: &Vec<path::PathBuf>,
    prefix: &Option<String>,
    suffix: &Option<String>,
) {
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
                let new_name = new_name.strip_suffix(suffix).unwrap().to_string();
                format!("{}.{}", new_name, ext)
            }
            None => new_name,
        };

        let new_path = file.with_file_name(new_name);
        std::fs::rename(file, new_path).unwrap();
    }
}
