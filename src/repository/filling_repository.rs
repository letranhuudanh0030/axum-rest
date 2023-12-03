use async_trait::async_trait;
use axum::{extract::Path, Json};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};

use crate::{
    model::filling_model::{CreateFillingModel, FillingModel},
    utils::api_response::Error,
    ModelManager,
};
use entity::filling::{self, Entity as Filling};

#[derive(Debug)]
pub struct FillingRepository {
    db: DatabaseConnection,
}

impl FillingRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { db: connection }
    }
}

#[async_trait]
pub trait AbstractFillingRepository {
    async fn find_all(&self) -> Result<Vec<FillingModel>, DbErr>;
    async fn find(&self, Path(id): Path<i32>) -> Result<FillingModel, DbErr>;
    async fn create(&self, Json(payload): Json<CreateFillingModel>) -> Result<(), DbErr>;
}

#[async_trait]
impl AbstractFillingRepository for FillingRepository {
    async fn find_all(&self) -> Result<Vec<FillingModel>, DbErr> {
        let filling: Vec<FillingModel> = Filling::find()
            .all(&self.db)
            .await?
            .into_iter()
            .map(|item| FillingModel {
                id: item.id,
                name: item.name,
            })
            .collect();

        Ok(filling)
    }

    async fn find(&self, Path(id): Path<i32>) -> Result<FillingModel, DbErr> {
        let filling = Filling::find_by_id(id).one(&self.db).await?;
        if let Some(filling) = filling {
            Ok(FillingModel {
                id: filling.id,
                name: filling.name,
            })
        } else {
            Err(DbErr::Custom(Error::NOT_FOUND.to_string()))
        }
    }

    async fn create(&self, Json(payload): Json<CreateFillingModel>) -> Result<(), DbErr> {
        let _filling = Filling::insert(filling::ActiveModel {
            name: Set(payload.name),
            ..Default::default()
        })
        .exec(&self.db)
        .await?;

        Ok(())
    }

    // pub async fn update(
    //     &self,
    //     Path(id): Path<i32>,
    //     Json(payload): Json<UpdateFillingModel>,
    // ) -> Result<(), DbErr> {
    //     let db = &mm.db;

    //     let mut cake: entity::cake::ActiveModel = Cake::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom(Error::NOT_FOUND.to_string()))?
    //         .into();

    //     cake.name = Set(payload.name);

    //     let _update = Cake::update(cake).exec(db).await?;

    //     Ok(())
    // }

    // pub async fn delete(&self, Path(id): Path<i32>) -> Result<(), DbErr> {
    //     let db = &mm.db;

    //     let cake = Cake::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom(Error::NOT_FOUND.to_string()))?;

    //     let _delete = Cake::delete_by_id(cake.id).exec(db).await?;

    //     Ok(())
    // }
}
