use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV6};
use axum::Router;
use axum::routing::get;
use sqlx::postgres::PgPoolOptions;
use tracing::error;
use tracing::log::info;
use crate::routes::routes_paths::AppRoute;
use tracing_subscriber;
use crate::controllers::video_controller::{stream_video};
mod routes;
mod controllers;
mod helpers;
mod db;

use dotenv::dotenv;
use serde::de::Unexpected::Str;
use crate::helpers::env_helper;
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() {
    // Логгер
    tracing_subscriber::FmtSubscriber::new();
    tracing_subscriber::fmt()
        // Мб потом убрать
        .compact()
        .init();
    // берев env из файла
    let env_file_name = String::from(".env");
    dotenv::from_filename(env_file_name).expect("No .env file in directory proj");
    dotenv().ok();

    // connect базы
    set_db_connection().await;
    // старт приложения
    let app = Router::new()
        .route("/", get(hp_root))
        .route(AppRoute::path(&AppRoute::Video), get(stream_video));
    let key_env: &str = "APP_PORT";
    let port: u16 = env_helper::read_env(key_env);

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let listener = tokio::net::TcpListener::bind(&socket).await.unwrap();

    info!("Server starting on http://{}", socket);
    info!("Local access: http://localhost:{}", port);
    //эндпоинты приложения
    info!("↓ api endpoints ↓");
    AppRoute::generic_iterator(|path: AppRoute| {info!("{}",AppRoute::path(&path))});
    axum::serve(listener, app).await.unwrap();

}

async fn set_db_connection() {
    info!("Try connect to db");
    let pg_port_env: &str = "PG_PORT";
    let port:u16 = env_helper::read_env(pg_port_env);
    let pg_table_env: &str = "PG_TABLE";
    let table: String = env_helper::read_env(pg_table_env);
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        "postgres",
        "postgres",
        "localhost",
        port,
        table
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        // базу надо ручками создать
        .connect(&connection_string).await;
    // секунд 20 пытается подключится при первичном запуске проекта если базы нет
    match pool {
        Ok(..)=>{info!("DB Connected")}
        // если не получилось законнектится паникуем
        Err(..) => {panic!("errWithDBConnection")}
    }
}

async fn hp_root() -> &'static str {
    "App working"
}