use sea_orm::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "brands")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: u32,
    #[sea_orm(unique)]
    name: String,
    description: Option<String>,
}

impl ActiveModelBehavior for ActiveModel {}
