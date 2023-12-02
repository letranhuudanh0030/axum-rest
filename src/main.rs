use axum::Router;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod controller;
mod middleware;
mod model;
mod repository;
mod route;
mod service;
mod utils;

pub use self::config::database;
pub use self::model::ModelManager;
pub use self::route::{route_static, routes};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize ModelManager.
    let mm = ModelManager::new().await.unwrap();

    // region:    --- Define route
    let routes_all = Router::new()
        .nest("/v1", routes(mm.clone()))
        .layer(CookieManagerLayer::new())
        .fallback_service(route_static::serve_dir());
    // endRegion: --- Define route

    // region:    --- Start server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endRegion: --- Start server
}
