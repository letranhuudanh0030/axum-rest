use axum::{routing::get, Router};

use crate::controller::cake_controller;
use crate::ModelManager;

pub fn routes() -> Router<ModelManager> {
    Router::new()
        .route(
            "/",
            get(cake_controller::list).post(cake_controller::create),
        )
        .route(
            "/:id",
            get(cake_controller::detail)
                .put(cake_controller::update)
                .delete(cake_controller::delete),
        )
}
