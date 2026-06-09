use axum::{
    Json,  extract::{Path }, http::StatusCode
};
use std::{collections::HashMap};

use crate::file_explorer;

pub async fn list_folders() -> Json<HashMap<String, Vec<String>>> {
    println!("Listing folders...");
    let mut response = HashMap::new();
    let folders: Vec<String> = file_explorer::get_folders();
    response.insert("names".to_string(), folders);
    Json(response)
}

pub async fn list_files(
    Path(folder_name): Path<String>,
) -> Result<Json<Vec<file_explorer::FileInfo>>, (StatusCode, String)>{
    println!("Listing files...");
    if !file_explorer::folder_is_allowed(&folder_name) {
        println!("{} to tiene permiso o no existe", folder_name);
        return Err((
            StatusCode::BAD_REQUEST, 
            "Folder doesn't exists or doesn't have access".to_string()
        ));
    }

    let mut files = file_explorer::list_files(&folder_name);
    files.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(Json(files))

}



