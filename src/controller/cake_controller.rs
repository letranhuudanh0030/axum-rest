use crate::{
    model::cake_model::{CakeModel, CreateCakeModel, UpdateCakeModel},
    service::CakeService,
    utils::api_error::APIResponse,
    ModelManager,
};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
};

use serde_json::{json, Value};

pub async fn list(State(mm): State<ModelManager>) -> impl IntoResponse {
    let result = CakeService::get_all(&mm)
        .await
        .into_iter()
        .map(serde_json::to_value)
        .collect();

    match result {
        Ok(data) => {
            // On success
            Json(APIResponse {
                data: Some(data),
                message: "success".to_string(),
                status: true,
                validate_error: None,
            })
        }
        Err(ex) => {
            // On error
            Json(APIResponse {
                data: None,
                message: ex.to_string(),
                status: false,
                validate_error: None,
            })
        }
    }
}

pub async fn detail(
    State(mm): State<ModelManager>,
    id: Path<i32>,
) -> Result<Json<CakeModel>, APIResponse> {
    let data = CakeService::get_by_id(&mm, id)
        .await
        .map_err(|ex| APIResponse {
            data: None,
            message: ex.to_string(),
            status: false,
            validate_error: None,
        })?;
    Ok(Json(data))
}

pub async fn create(
    State(mm): State<ModelManager>,
    payload: Json<CreateCakeModel>,
) -> Result<Json<Value>, APIResponse> {
    let _insert = CakeService::create(&mm, payload)
        .await
        .map_err(|ex| APIResponse {
            data: None,
            message: ex.to_string(),
            status: false,
            validate_error: None,
        })?;

    let body = Json(json!({
        "result": {
            "success": true
        },
    }));

    Ok(body)
}

pub async fn update(
    State(mm): State<ModelManager>,
    id: Path<i32>,
    payload: Json<UpdateCakeModel>,
) -> Result<Json<Value>, APIResponse> {
    let _update = CakeService::update(&mm, id, payload)
        .await
        .map_err(|ex| APIResponse {
            data: None,
            message: ex.to_string(),
            status: false,
            validate_error: None,
        })?;

    let body = Json(json!({
        "result": {
            "success": true
        },
    }));

    Ok(body)
}

pub async fn delete(
    State(mm): State<ModelManager>,
    id: Path<i32>,
) -> Result<Json<Value>, APIResponse> {
    let _delete = CakeService::delete(&mm, id)
        .await
        .map_err(|ex| APIResponse {
            data: None,
            message: ex.to_string(),
            status: false,
            validate_error: None,
        })?;

    let body = Json(json!({
        "result": {
            "success": true
        },
    }));

    Ok(body)
}
