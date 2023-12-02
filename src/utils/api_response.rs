use axum::{http::header, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Serialize)]
pub struct APIResponse<T> {
    pub data: Option<T>,
    pub message: String,
    pub status: bool,
    pub validate_error: Option<Vec<Value>>,
}

impl<T: Serialize> IntoResponse for APIResponse<T> {
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
    CREATE_FAIL,
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

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum Info {
    SUCCESS,
    CREATE_SUCCESS,
    UPDATE_SUCCESS,
    DELETE_SUCCESS,
}

// region:    --- Info Boilerplate
impl core::fmt::Display for Info {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
// endregion: --- Info Boilerplate
