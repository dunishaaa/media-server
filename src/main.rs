use axum::{
    Json, Router,
    extract::{Path, Query},
    http::{ StatusCode},
    routing::get, 
};
use core::{convert::From, result::Result};
use serde::{Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tower_http::{cors::{Any, CorsLayer}};
use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;
use walkdir::WalkDir;
use once_cell::sync::Lazy;
use std::fs;

#[derive(Serialize)]
struct FileInfo {
    name: String,
    size_mb: f64,
    extension: String,
    path: String,
    modified: String,
}

const CONFIG_PATH: &str = "./config.txt";

static FOLDERS: Lazy<HashMap<String, PathBuf>> = Lazy::new(|| {
    let mut m = HashMap::new();
    let allowed_folders = parse_config();
    for folder in allowed_folders {
        let folder_name: Vec<&str> = folder.rsplit('/').collect();
        m.insert(folder_name[0].to_string(), PathBuf::from(&folder));
    }
    m
});

fn parse_config() -> Vec<String>{
    let mut folders: Vec<String> = vec![];
    let contents = fs::read_to_string(CONFIG_PATH).expect(format!("Unable to read config file at {}", CONFIG_PATH).as_str());
    folders = contents.split('\n').map(|x| x.to_string()).collect();
    println!("{:?}", folders);
    folders
}
async fn list_folders() -> Json<HashMap<String, Vec<String>>> {
    println!("Listing folders...");
    let mut response = HashMap::new();
    let folders: Vec<String> = FOLDERS.keys().map(|x| x.to_string()).collect();
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
    parse_config();
    tracing_subscriber::fmt::init();

    let _ip_addr= "192.168.1.80";
    let port= "3000";
    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api = Router::new()
        .route("/folders", get(list_folders))
        .route("/files/{folder}", get(list_files));

    let mut app = Router::new()
        .nest("/api", api)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .fallback_service(
            ServeDir::new("./frontend/dist")
            .append_index_html_on_directories(true)
        );
    
    for (folder_name, path) in FOLDERS.iter() {
        let temp = format!("/download/{}", &folder_name[..]);
        app = app.nest_service(
            &temp,
            ServeDir::new(path)
        );
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap(); 

    println!("Servidor corriendo en: "); 
    println!("   - Local: http://localhost:{}", port);
    println!("   - Red: http://{}:{}", local_ip, port);
    println!("documentacion auto en http://{}:{}/ ", local_ip, port);
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