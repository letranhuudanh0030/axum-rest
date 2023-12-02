use axum::{routing::post, Router};

use crate::ModelManager;

pub fn routes() -> Router<ModelManager> {
    Router::new()
        .route("/login", post(println!("login")))
        .route("/logout", post(println!("logout")))
}
