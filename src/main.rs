use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;
use sqlx::postgres::PgPoolOptions;
use tracing::log::info;
use crate::routes::routes_paths::AppRoute;
use tracing_subscriber;
use crate::controllers::video_controller::{stream_video};
mod routes;
mod controllers;

#[tokio::main]
async fn main() {

    let subscriber = tracing_subscriber::FmtSubscriber::new();
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .init();
    let app = Router::new()
        .route("/", get(root))
        .route(AppRoute::path(&AppRoute::Video), get(stream_video));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        // базу надо ручками создать
        .connect("postgres://postgres:postgres@localhost:5555/video_system").await;
    // секунд 20 пытается подключится при первичном запуске проекта
    match pool {
        Ok(..)=>{println!("DB connected")}
        // если не получилось законнектится паникуем
        Err(..) => {panic!("errWithDBConnection")}
    }

    let key_env = String::from("PORT");
    let port: u16 = std::env::var(key_env)
        .unwrap_or_else(|_| "3000".to_string())
        .parse() // Parse the port string into a u16
        .expect("Failed to parse PORT");

    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    info!("Server starting on http://{}", address);
    info!("Local access: http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();

}

async fn root() -> &'static str {
    "Hello World, from Axum!"
}