pub use sea_orm_migration::prelude::*;

mod m20231130_000001_create_cake_table;
mod m20231130_000002_create_filling_table;
mod m20231130_000003_create_fruit_table;
mod m20231130_000004_create_cake_filling_table;
mod m20231130_070544_add_primary_key_cake_filling_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231130_000001_create_cake_table::Migration),
            Box::new(m20231130_000002_create_filling_table::Migration),
            Box::new(m20231130_000003_create_fruit_table::Migration),
            Box::new(m20231130_000004_create_cake_filling_table::Migration),
            Box::new(m20231130_070544_add_primary_key_cake_filling_table::Migration),
        ]
    }
}
