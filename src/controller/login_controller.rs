use crate::{
    model::login_model::LoginPayload,
    utils::error::{Error, Result},
    ModelManager,
};
use axum::{extract::State, Json};
use serde_json::{json, Value};
use tower_cookies::Cookies;

pub async fn login(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

pub async fn logout() -> Result<Json<Value>> {
    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}
