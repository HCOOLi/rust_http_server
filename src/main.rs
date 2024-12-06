use axum::{response::Html, routing::{get,post}, Router};
use tower_http::services::ServeDir;
use tokio::fs;
use std::path::Path;
use tokio::io::AsyncReadExt;
use serde::Deserialize;


#[derive(Deserialize)]
struct MyData {
    message: String,
    email: String,
}

#[tokio::main]
async fn main() {

    let app = Router::new()
        .nest_service("/", ServeDir::new("./"));
        // .route("/qzchess", get(qzchess_handler));
        // .nest_service("/qzchess/assets", ServeDir::new("qzchess/assets"))
        // .nest_service("/qzchess/out", ServeDir::new("qzchess/out"))
        // .route("/rougelike", get(rougelike_handler))
        // .nest_service("/rougelike/assets", ServeDir::new("rougelike/assets"))
        // .nest_service("/rougelike/out", ServeDir::new("rougelike/out"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Html<String> {
    let file_path = Path::new("index.html");
    let mut file = fs::File::open(file_path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    Html(contents)
}

async fn qzchess_handler() -> Html<String> {
    let file_path = Path::new("qzchess/index.html");
    let mut file = fs::File::open(file_path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    Html(contents)
}

async fn rougelike_handler() -> Html<String> {
    let file_path = Path::new("rougelike/index.html");
    let mut file = fs::File::open(file_path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    Html(contents)
}
