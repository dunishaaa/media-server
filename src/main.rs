use axum::{
    Json, Router,
    extract::{Path, Query},
    http::{Response, StatusCode, Uri, header},
    response::{IntoResponse },
    routing::get, serve::{self, Listener},
};
use core::{convert::From, result::Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tower_http::{cors::{Any, CorsLayer}};
use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;
use walkdir::WalkDir;
use once_cell::sync::Lazy;

#[derive(Serialize)]
struct FileInfo {
    name: String,
    size_mb: f64,
    extension: String,
    path: String,
    modified: String,
}

const AUDIO_DIR: &str = "/home/dunishaaa/media/audios";
const VIDEO_DIR: &str = "/home/dunishaaa/media/videos";
const BOOKS_DIR: &str = "/home/dunishaaa/media/books";
const RANDOM_DIR: &str = "/home/dunishaaa/media/random";

static FOLDERS: Lazy<HashMap<&'static str, PathBuf>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("audios", PathBuf::from(AUDIO_DIR));
    m.insert("videos", PathBuf::from(VIDEO_DIR));
    m.insert("books", PathBuf::from(BOOKS_DIR));
    m.insert("random", PathBuf::from(RANDOM_DIR));
    m
});

async fn list_folders() -> Json<HashMap<String, Vec<String>>> {
    println!("Listing folders...");
    let mut response = HashMap::new();
    let folders: Vec<String> = FOLDERS.keys().map(|&k| k.to_string()).collect();
    response.insert("names".to_string(), folders);
    Json(response)
}

async fn list_files(
    Path(folder_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<FileInfo>>, (StatusCode, String)>{

    println!("Listing files...");

    let folder_path = FOLDERS
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

            files.push(FileInfo { 
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

#[tokio::main]
async fn main(){
    tracing_subscriber::fmt::init();

    let IP_ADDR= "192.168.1.80";
    let PORT = "3000";
    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api = Router::new()
        .route("/folders", get(list_folders))
        .route("/files/{folder}", get(list_files));

    let app = Router::new()
        .nest("/api", api)
        .nest_service("/download", ServeDir::new(VIDEO_DIR))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .fallback_service(
            ServeDir::new("./frontend/dist")
            .append_index_html_on_directories(true)
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap(); 

    println!("Servidor corriendo en: "); 
    println!("   - Local: http://localhost:{}", PORT);
    println!("   - Red: http://{}:{}", local_ip, PORT);
    println!("documentacion auto en http://{}:{}/ ", local_ip, PORT);
    axum::serve(listener, app).await.unwrap();
}

fn get_local_ip() -> Option<String>{
    use std::net::{UdpSocket};

    let socket = UdpSocket::bind("0.0.0.0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let local_addr = socket.local_addr().ok()?;
    match local_addr.ip() {
        std::net::IpAddr::V4(ip) => Some(ip.to_string()),
        std::net::IpAddr::V6(_) => None,
    }
}