mod routes;

use axum::{routing, Router};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::routes::pinyin_input;

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("./assets/static");

    let app = Router::new()
        .route("/pinyin-input", routing::get(pinyin_input::route))
        .nest_service("/assets", serve_dir);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
