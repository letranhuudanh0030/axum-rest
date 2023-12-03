use crate::{
    model::cake_model::{CreateCakeModel, UpdateCakeModel},
    service::CakeService,
    utils::api_response::{APIResponse, Info},
    ModelManager,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

pub async fn list(State(mm): State<ModelManager>) -> impl IntoResponse {
    let result = CakeService::get_all(&mm).await;

    let (data, is_ok, msg) = match result {
        Ok(data) => (Some(data), true, Info::SUCCESS.to_string()),
        Err(ex) => (None, false, ex.to_string()),
    };

    Json(APIResponse {
        data: data,
        message: msg,
        status: is_ok,
        validate_error: None,
    })
}

pub async fn detail(State(mm): State<ModelManager>, id: Path<i32>) -> impl IntoResponse {
    let result = CakeService::get_by_id(&mm, id).await;
    let (data, is_ok, msg) = match result {
        Ok(data) => (Some(data), true, Info::SUCCESS.to_string()),
        Err(ex) => (None, false, ex.to_string()),
    };

    Json(APIResponse {
        data: data,
        message: msg,
        status: is_ok,
        validate_error: None,
    })
}

pub async fn create(
    State(mm): State<ModelManager>,
    payload: Json<CreateCakeModel>,
) -> impl IntoResponse {
    let insert = CakeService::create(&mm, payload).await;

    let (data, is_ok, msg) = match insert {
        Ok(data) => (Some(data), true, Info::CREATE_SUCCESS.to_string()),
        Err(ex) => (None, false, ex.to_string()),
    };

    Json(APIResponse {
        data: data,
        message: msg,
        status: is_ok,
        validate_error: None,
    })
}

pub async fn update(
    State(mm): State<ModelManager>,
    id: Path<i32>,
    payload: Json<UpdateCakeModel>,
) -> impl IntoResponse {
    let update = CakeService::update(&mm, id, payload).await;

    let (data, is_ok, msg) = match update {
        Ok(data) => (Some(data), true, Info::UPDATE_SUCCESS.to_string()),
        Err(ex) => (None, false, ex.to_string()),
    };

    Json(APIResponse {
        data: data,
        message: msg,
        status: is_ok,
        validate_error: None,
    })
}

pub async fn delete(State(mm): State<ModelManager>, id: Path<i32>) -> impl IntoResponse {
    let delete = CakeService::delete(&mm, id).await;

    let (data, is_ok, msg) = match delete {
        Ok(data) => (Some(data), true, Info::DELETE_SUCCESS.to_string()),
        Err(ex) => (None, false, ex.to_string()),
    };

    Json(APIResponse {
        data: data,
        message: msg,
        status: is_ok,
        validate_error: None,
    })
}
