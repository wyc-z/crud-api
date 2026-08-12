use super::entities::*;
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
};
use sea_orm_migration::prelude::*;
pub struct Migration;
use rust_decimal_macros::dec;
impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260812_091924_initial_data"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        let db = _manager.get_connection();

        category::ActiveModel {
            id: NotSet,
            name: Set(String::from("Laptops")),
            description: Set(Some(String::from("Computadoras Portátiles"))),
        }
        .insert(db)
        .await?;

        brand::ActiveModel {
            id: NotSet,
            name: Set(String::from("Lenovo")),
            description: Set(None),
        }
        .insert(db)
        .await?;

        product::ActiveModel {
            id: NotSet,
            name: Set(String::from("ThinkPad T480")),
            brand_id: Set(Some(1)),
            category_id: Set(1),
            cost: Set(dec!(4000.74)),
            price: Set(dec!(7000.97)),
            stock: Set(5),
            sku: Set(String::from("LAP-LEN480")),
            description: Set(None),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        product::Entity::delete_by_sku(String::from("LAP-LEN480"))
            .exec(db)
            .await?;
        brand::Entity::delete_by_name(String::from("Lenovo"))
            .exec(db)
            .await?;
        category::Entity::delete_by_name(String::from("Laptops"))
            .exec(db)
            .await?;
        Ok(())
    }
}
