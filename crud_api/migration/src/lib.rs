pub use sea_orm_migration::prelude::*;

pub mod entities;

mod m20260812_082339_create_tables;
mod m20260812_091924_initial_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260812_082339_create_tables::Migration),
            Box::new(m20260812_091924_initial_data::Migration),
        ]
    }
}
