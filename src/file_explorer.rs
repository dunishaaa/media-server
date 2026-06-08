use serde::Serialize;
use std::path::PathBuf;
use std::collections::HashMap;
use once_cell::sync::Lazy;

use crate::config::parse_config;

#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub size_mb: f64,
    pub extension: String,
    pub path: String,
    pub modified: String,
}
pub static FOLDERS: Lazy<HashMap<String, PathBuf>> =  Lazy::new(|| {
    println!("Loading folders...");
    let mut m = HashMap::new();
    let allowed_folders = parse_config();
        for folder in allowed_folders {
            let folder_name: Vec<&str> = folder.rsplit('/').collect();
            m.insert(folder_name[0].to_string(), PathBuf::from(&folder));
    }
    m
});

