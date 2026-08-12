use sea_orm::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub brand_id: Option<u32>,
    #[sea_orm(belongs_to, from = "brand_id", to = "id")]
    pub brand: BelongsTo<Option<super::brand::Entity>>,
    pub category_id: u32,
    #[sea_orm(belongs_to, from = "category_id", to = "id")]
    pub category: BelongsTo<super::category::Entity>,
    pub cost: Decimal,
    pub price: Decimal,
    pub stock: u32,
    #[sea_orm(unique)]
    pub sku: String,
    pub description: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}
