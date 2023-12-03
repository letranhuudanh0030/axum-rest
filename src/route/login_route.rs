use axum::{routing::post, Router};

use crate::{controller::login_controller, ModelManager};

pub fn routes() -> Router<ModelManager> {
    todo!("login routes")
    // Router::new().route("/login", post(login_controller::login))
    // .route("/logout", post(login_controller::logout))
}
