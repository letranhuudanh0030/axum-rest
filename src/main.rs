use axum::Router;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod controller;
mod error;
mod middleware;
mod model;
mod repository;
mod route;
mod service;
mod utils;

use crate::model::AppState;

pub use self::config::database;
pub use self::error::{Error, Result};
pub use self::route::{route_static, routes};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize ModelManager.
    let state = AppState::new().await?;

    // region:    --- Define route
    let routes_all = Router::new()
        .merge(routes(state.clone()))
        .fallback_service(route_static::serve_dir());
    // endRegion: --- Define route

    // region:    --- Start server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    // endRegion: --- Start server
    Ok(())
}
