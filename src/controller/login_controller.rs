use axum::{extract::State, Json};
use serde_json::Value;
use tower_cookies::Cookies;

use crate::{model::login_model::LoginPayload, utils::api_response::Error, ModelManager};

pub async fn login(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>, Error> {
    todo!("login");
}

pub async fn logout() -> Result<Json<Value>, Error> {
    todo!("logout");
}
