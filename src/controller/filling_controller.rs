use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    repository::filling_repository::FillingRepository,
    service::filling_service::FillingService,
    utils::api_response::{APIResponse, Info},
    ModelManager,
};

pub async fn list(State(mm): State<ModelManager>) -> impl IntoResponse {
    let result = FillingService::new(&FillingRepository::new(mm.db))
        .get_all()
        .await;

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
