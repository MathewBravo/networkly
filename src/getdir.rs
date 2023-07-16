use std::{
    env,
    path::{Path, PathBuf},
};

pub fn get_home_dir() -> String {
    let home_dir = env::var("HOME").expect("Failed to get your home directory");
    home_dir
}

pub fn get_file_path(dir: &str, file_name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(dir);
    path.push(file_name);
    path
}

pub fn get_networkly_connections_dir() -> PathBuf {
    let home_dir = get_home_dir();
    let networkly_dir = Path::new(&home_dir).join(".networkly/connections");

    networkly_dir
}
