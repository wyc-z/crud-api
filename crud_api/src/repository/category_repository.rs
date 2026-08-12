use super::Repository;
use crate::{model::category::Category, service::error::AppError};
use async_trait::async_trait;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    prelude::*,
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: u32,
    #[sea_orm(unique)]
    name: String,
    description: Option<String>,
}
impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Category {
    fn from(model: Model) -> Self {
        Category::new(Some(model.id), model.name, model.description)
    }
}
impl From<Category> for ActiveModel {
    fn from(category: Category) -> Self {
        ActiveModel {
            id: NotSet,
            name: Set(String::from(category.name())),
            description: Set(category.description().map(str::to_owned)),
        }
    }
}

pub struct SeaCategoryRepository {
    conn: DatabaseConnection,
}
impl SeaCategoryRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}
#[async_trait]
impl Repository<Category, u32> for SeaCategoryRepository {
    async fn save(&self, entity: Category) -> Result<Category, AppError> {
        let model: ActiveModel = entity.into();
        Ok(model.insert(&self.conn).await?.into())
    }

    async fn update(&self, entity: Category) -> Result<Category, AppError> {
        let id = entity
            .id()
            .ok_or(AppError::NotFound(String::from("Category")))?;
        if Entity::find_by_id(id).one(&self.conn).await?.is_none() {
            return Err(AppError::NotFound(String::from("Category not found!")));
        }
        let mut model: ActiveModel = entity.into();
        model.id = Set(id);
        let active = Entity::update(model).exec(&self.conn).await?;
        Ok(active.into())
    }
    async fn get_by_id(&self, id: u32) -> Result<Option<Category>, AppError> {
        let model = Entity::find_by_id(id).one(&self.conn).await?;
        Ok(model.map(Into::into))
    }
    async fn get_all(&self) -> Result<Vec<Category>, AppError> {
        let models = Entity::find().all(&self.conn).await?;
        let brands = models.into_iter().map(Into::into).collect();
        Ok(brands)
    }
    async fn delete_by_id(&self, id: u32) -> Result<(), AppError> {
        let result = Entity::delete_by_id(id).exec(&self.conn).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound(String::from("Category not found!")));
        }
        Ok(())
    }
}
