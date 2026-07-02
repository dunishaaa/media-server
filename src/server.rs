use axum::{
    Json,  extract::Path, http::StatusCode
};
use serde::Deserialize;
use std::{collections::HashMap};

use crate::file_explorer;
use crate::yt_dlp;
use crate::yt_dlp::Download;

pub async fn list_folders() -> Json<HashMap<String, Vec<String>>> {
    println!("Listing folders...");
    let mut response = HashMap::new();
    let folders: Vec<String> = file_explorer::get_folder_names();
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
#[derive(Deserialize)]
enum MediaType {
    VIDEO,
    AUDIO,
}

#[derive(Deserialize)]
pub struct Media {
    media_type: MediaType,
    height_quality: String,
    url: String,
    extension: String,
    download_path: String,
}


pub async fn download_video(
    Json(payload): Json<Media>,
) -> Result<(), (StatusCode, String)>{
    let media_type = match payload.media_type {
        MediaType::VIDEO => yt_dlp::MediaType::VIDEO,
        MediaType::AUDIO => yt_dlp::MediaType::AUDIO,
    };
    let mut media = yt_dlp::Media::new(
        media_type,
        &payload.height_quality,
        &payload.url,
        &payload.extension,
        &payload.download_path
    );
    let status = media.download();
    match status {
        Ok(_) => Ok(()),
        Err(err) => Err(( StatusCode::BAD_REQUEST, err.to_string()))
    }

}

pub async fn download_playlist(){}
