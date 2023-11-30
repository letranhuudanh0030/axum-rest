use super::m20231130_000001_create_cake_table::Cake;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Fruit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Fruit::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Fruit::Name).string().not_null())
                    .col(ColumnDef::new(Fruit::CakeId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-fruit-cake")
                            .from(Fruit::Table, Fruit::CakeId)
                            .to(Cake::Table, Cake::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Fruit::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Fruit {
    Table,
    Id,
    Name,
    CakeId,
}
