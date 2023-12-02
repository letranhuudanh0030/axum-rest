use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize)]
pub struct APIResponse {
    pub data: Option<Vec<Value>>,
    pub message: String,
    pub status: bool,
    pub validate_error: Option<Vec<Value>>,
}

impl IntoResponse for APIResponse {
    fn into_response(self) -> axum::response::Response {
        (
            [(header::CONTENT_TYPE, "application/json")],
            Json(
                json!({"data": self.data, "status": self.status,"message": self.message, "validate_error": self.validate_error }),
            ),
        ).into_response()
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum Error {
    NO_DATA,
    INSERT_FAIL,
    NOT_FOUND,
    UPDATE_FAIL,
    DELETE_FAIL,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
