use super::m20231130_000001_create_cake_table::Cake;
use super::m20231130_000003_create_fruit_table::Fruit;
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
                    .table(CakeFilling::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CakeFilling::CakeId).integer())
                    .col(ColumnDef::new(CakeFilling::FillingId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cake_filling-cake")
                            .from(CakeFilling::Table, CakeFilling::CakeId)
                            .to(Cake::Table, Cake::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cake_filling-filling")
                            .from(CakeFilling::Table, CakeFilling::FillingId)
                            .to(Fruit::Table, Fruit::Id)
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
            .drop_table(Table::drop().table(CakeFilling::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CakeFilling {
    Table,
    CakeId,
    FillingId,
}
