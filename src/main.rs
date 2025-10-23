use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV6};
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, Method};
use axum::Router;
use axum::routing::{get, post};
use sqlx::postgres::PgPoolOptions;
use tracing::error;
use tracing::log::info;
use crate::routes::main_routes::AppRoute;
use tracing_subscriber;
use crate::controllers::video_controller::{stream_video, upload_video_chunked};
mod routes;
mod controllers;
mod helpers;
mod db;

use dotenv::dotenv;
use serde::de::Unexpected::Str;
use sqlx::{PgPool, Pool, Postgres};
use crate::helpers::env_helper;
use strum::IntoEnumIterator;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    pool: PgPool
}

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

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST]);

    // connect базы
    let pool = set_db_connection().await;
    let state = AppState { pool };
    run_migrations(&state.pool).await.expect("DB migration err");
    // старт приложения
    let app = Router::new()
        .route("/", get(hp_root))
        .route(&AppRoute::path(&AppRoute::VideoGet), get(stream_video))
        .route(&AppRoute::path(&AppRoute::VideoUpload), post(upload_video_chunked))
        // лимит в 250 мб
        .layer(DefaultBodyLimit::max(1024*1024*250))
        .layer(cors);
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

async fn set_db_connection() -> Pool<Postgres> {
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
        Ok(pl)=>{
            info!("DB Connected");
            pl
        }
        // если не получилось законнектится паникуем
        Err(..) => {panic!("errWithDBConnection")}
    }
}

async fn run_migrations(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            info!("✅ Migrations applied successfully");
            Ok(())
        }
        Err(e) => {
            error!("❌ Failed to run migrations: {}", e);
            Err(Box::new(e))
        }
    }
}

async fn hp_root() -> &'static str {
    "App working"
}