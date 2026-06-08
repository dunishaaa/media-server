use axum::{
    Router,
    routing::get, 
};
use tower_http::{cors::{Any, CorsLayer}};
use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;

mod file_explorer;
mod config;    
mod server;

#[tokio::main]
async fn main(){
    tracing_subscriber::fmt::init();

    let _ip_addr= "192.168.1.80";
    let port= "3000";

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api = Router::new()
        .route("/folders", get(server::list_folders))
        .route("/files/{folder}", get(server::list_files));

    let mut app = Router::new()
        .nest("/api", api)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .fallback_service(
            ServeDir::new("./frontend/dist")
            .append_index_html_on_directories(true)
        );
    
//    server::create_folder_paths(app, folders));
    for (folder_name, path) in file_explorer::FOLDERS.iter() {
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
    axum::serve(listener, app).await.unwrap();
}
