use serde::Serialize;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use walkdir::WalkDir;

use crate::config;

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
    let allowed_folders = get_valid_folders();
        for folder in allowed_folders {
            let folder_name: Vec<&str> = folder.rsplit('/').collect();
            m.insert(folder_name[0].to_string(), PathBuf::from(&folder));
    }
    m
});


fn get_valid_folders() -> Vec<String> {
    let allowed_folders = config::parse_config();
    let mut valid_folders = vec![];

    for folder in allowed_folders {
        if Path::new(&folder).exists(){
            valid_folders.push(folder);
        }else{
            println!("folder -> {} doesn't exists", folder);
        }
    }
    valid_folders
}

pub fn get_folder_names() -> Vec<String>{
    FOLDERS.keys()
        .map(|x| x.to_string())
        .collect()
}
pub fn folder_is_allowed(folder_name: &str) -> bool {
    return FOLDERS.contains_key(folder_name);
}

pub fn list_files(folder_name: &str) -> Vec<FileInfo>{
    let mut files = vec![];
    let folder_path = FOLDERS 
            .get(folder_name)
            .unwrap();

    for entry in WalkDir::new(folder_path)
        .sort_by_file_name()
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file(){

            let extension = entry.
                path().
                extension().
                map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            let name = entry.path().file_name().unwrap().to_string_lossy().to_string();
            let metadata = path.metadata().unwrap();
            let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
            let modified = metadata
                .modified()
                .ok()
                .and_then(|m| m.elapsed().ok())
                .map(|d| format!("{} days ago", d.as_secs() / 86400))
                .unwrap_or_else(|| "unknown".to_string());
            files.push(FileInfo {
                name,
                size_mb: (size_mb *100.0).round() / 100.0,
                extension,
                path: path.to_string_lossy().to_string(),
                modified
            });
        }
    }
    files
}
