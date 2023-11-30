// use crate::controller::cake_controller::list;
use super::error::Result;
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn routes() -> Router {
    let routes = Router::new()
        .route("/", get(list).post(println!("ok")))
        .route(
            "/:id",
            get(println!("ok"))
                .put(println!("ok"))
                .delete(println!("ok")),
        );

    Router::new().nest("/cake", routes)
}

async fn list() -> Result<JSON<Value>> {
    Ok(Json(json!({
        "data": [
            {
                "id": 1,
            }
        ]
    })))
}
