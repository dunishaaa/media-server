

use axum::{
    Router,
    routing::get, 
};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tower_http::services::ServeDir;
use local_ip_address::local_ip;

mod file_explorer;
mod config;    
mod server;
mod yt_dlp;

#[tokio::main]
async fn main(){

    tracing_subscriber::fmt::init();

    let _ = yt_dlp::test();

    let _ = config::write_ip();
    let local_ip = local_ip().unwrap();

    let _ip_addr= "0.0.0.0";
    let port= "3000";

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
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
        let temp = format!("/download/{}", folder_name);
        app = app.nest_service(
            &temp,
            ServeDir::new(path)
        );
    }

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap(); 

    println!("Servidor corriendo en: "); 
    println!("   - Local: http://{}:{}", local_ip, port);
    axum::serve(listener, app).await.unwrap();
}
