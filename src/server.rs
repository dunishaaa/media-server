use axum::{
    Json,  extract::{Path, Query}, http::StatusCode
};
use walkdir::WalkDir;
use std::{collections::HashMap};

use crate::file_explorer;

pub async fn list_folders() -> Json<HashMap<String, Vec<String>>> {
    println!("Listing folders...");
    let mut response = HashMap::new();


    let folders: Vec<String> = file_explorer::FOLDERS.keys().map(|x| x.to_string()).collect();
    response.insert("names".to_string(), folders);
    Json(response)
}

pub async fn list_files(
    Path(folder_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<file_explorer::FileInfo>>, (StatusCode, String)>{

    println!("Listing files...");


    let folder_path = file_explorer::FOLDERS 
        .get(folder_name.as_str())
        .ok_or((StatusCode::NOT_FOUND, "Carpeta no encontrada\n".to_string()))?;

    if !folder_path.exists(){
        return Ok(Json(vec![]));
    }

    let extension_filter = params.get("extension").map(|s| s.to_lowercase());
    let mut files = Vec::new();

    for entry in WalkDir::new(folder_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file(){
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let extension = path
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            if let Some(ref filter) = extension_filter {
                if extension != *filter {
                    continue;
                }
            }
            let metadata = path.metadata().unwrap();
            let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
            let modified = metadata
                .modified()
                .ok()
                .and_then(|m| m.elapsed().ok())
                .map(|d| format!("{} days ago", d.as_secs() / 86400))
                .unwrap_or_else(|| "unknown".to_string());

            files.push(file_explorer::FileInfo { 
                name, 
                size_mb: (size_mb * 100.0).round() / 100.0, 
                extension, path: path.to_string_lossy().to_string(), 
                modified 
            });
        }    


    }

    files.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(Json(files))

}



