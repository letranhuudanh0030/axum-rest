use sea_orm::DbErr;

pub mod cake_model;
pub mod filling_model;
pub mod login_model;

use crate::database::{new_db_pool, Db};

#[derive(Debug, Clone)]
pub struct ModelManager {
    pub db: Db,
}

impl ModelManager {
    /// Constructor
    pub async fn new() -> Result<Self, DbErr> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }
}
