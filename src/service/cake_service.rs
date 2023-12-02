use axum::extract::Path;
use axum::Json;
use serde_json::{json, Value};

use crate::model::cake_model::{CreateCakeModel, UpdateCakeModel};
use crate::utils::api_error::{APIResponse, Error};
use crate::ModelManager;

use crate::{model::cake_model::CakeModel, repository::CakeRepository};

#[derive(Debug)]
pub struct CakeService;

impl CakeService {
    pub async fn get_all(mm: &ModelManager) -> Result<Vec<CakeModel>, Error> {
        CakeRepository::find_all(mm)
            .await
            .map_err(|_| Error::NO_DATA)

        // let body_response = json!({
        //     "data": result
        // });

        // Ok(Json(body_response))
    }

    pub async fn get_by_id(mm: &ModelManager, id: Path<i32>) -> Result<CakeModel, Error> {
        CakeRepository::find(mm, id)
            .await
            .map_err(|_| Error::NOT_FOUND)
    }

    pub async fn create(mm: &ModelManager, payload: Json<CreateCakeModel>) -> Result<(), Error> {
        CakeRepository::create(mm, payload)
            .await
            .map_err(|_| Error::INSERT_FAIL)
    }

    pub async fn update(
        mm: &ModelManager,
        id: Path<i32>,
        payload: Json<UpdateCakeModel>,
    ) -> Result<(), Error> {
        CakeRepository::update(mm, id, payload)
            .await
            .map_err(|_| Error::UPDATE_FAIL)
    }

    pub async fn delete(mm: &ModelManager, id: Path<i32>) -> Result<(), Error> {
        CakeRepository::delete(mm, id)
            .await
            .map_err(|_| Error::DELETE_FAIL)
    }
}
