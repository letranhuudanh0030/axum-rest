use axum::{routing::get, Router};

use crate::controller::filling_controller;
use crate::ModelManager;

pub fn routes() -> Router<ModelManager> {
    Router::new().route("/", get(filling_controller::list))
}
