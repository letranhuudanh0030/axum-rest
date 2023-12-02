use crate::{
    model::cake_model::{CreateCakeModel, UpdateCakeModel},
    utils::api_response::Error,
    ModelManager,
};
use axum::{extract::Path, Json};
use entity::cake::{self, Entity as Cake};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};

use crate::model::cake_model::CakeModel;

#[derive(Debug)]
pub struct CakeRepository;

impl CakeRepository {
    pub async fn find_all(mm: &ModelManager) -> Result<Vec<CakeModel>, DbErr> {
        let db = &mm.db;
        let cakes: Vec<CakeModel> = Cake::find()
            .all(db)
            .await?
            .into_iter()
            .map(|item| CakeModel {
                id: item.id,
                name: item.name,
            })
            .collect();

        Ok(cakes)
    }

    pub async fn find(mm: &ModelManager, Path(id): Path<i32>) -> Result<CakeModel, DbErr> {
        let db = &mm.db;
        let cake = Cake::find_by_id(id).one(db).await?;
        if let Some(cake) = cake {
            Ok(CakeModel {
                id: cake.id,
                name: cake.name,
            })
        } else {
            Err(DbErr::Custom(Error::NOT_FOUND.to_string()))
        }
    }

    pub async fn create(
        mm: &ModelManager,
        Json(payload): Json<CreateCakeModel>,
    ) -> Result<(), DbErr> {
        let db = &mm.db;
        let _cake = Cake::insert(cake::ActiveModel {
            name: Set(payload.name),
            ..Default::default()
        })
        .exec(db)
        .await?;

        Ok(())
    }

    pub async fn update(
        mm: &ModelManager,
        Path(id): Path<i32>,
        Json(payload): Json<UpdateCakeModel>,
    ) -> Result<(), DbErr> {
        let db = &mm.db;

        let mut cake: entity::cake::ActiveModel = Cake::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom(Error::NOT_FOUND.to_string()))?
            .into();

        cake.name = Set(payload.name);

        let _update = Cake::update(cake).exec(db).await?;

        Ok(())
    }

    pub async fn delete(mm: &ModelManager, Path(id): Path<i32>) -> Result<(), DbErr> {
        let db = &mm.db;

        let cake = Cake::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom(Error::NOT_FOUND.to_string()))?;

        let _delete = Cake::delete_by_id(cake.id).exec(db).await?;

        Ok(())
    }
}
