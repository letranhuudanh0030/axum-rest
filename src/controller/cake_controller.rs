use axum::{extract::State, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde_json::{json, Value};

use crate::model::{cake, AppState};

use super::error::Result;

use crate::model::cake::Entity as Cake;

pub async fn list() -> Result<Json<Value>> {
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

// pub async fn detail() -> Result<()> {
//     println!("detail");
//     Ok(())
// }

// pub async fn create() -> Result<()> {
//     println!("create");
//     Ok(())
// }

// pub async fn update() -> Result<()> {
//     println!("update");
//     Ok(())
// }

// pub async fn delete() -> Result<()> {
//     println!("delete");
//     Ok(())
// }
