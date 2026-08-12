use super::Repository;
use crate::model::brand::Brand;
use crate::service::error::AppError;
use async_trait::async_trait;
use sea_orm::ActiveValue::{NotSet, Set};
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

impl From<Model> for Brand {
    fn from(model: Model) -> Self {
        Brand::new(Some(model.id), model.name, model.description)
    }
}
impl From<Brand> for ActiveModel {
    fn from(brand: Brand) -> Self {
        ActiveModel {
            id: NotSet,
            name: Set(String::from(brand.name())),
            description: Set(brand.description().map(str::to_owned)),
        }
    }
}

pub struct SeaBrandRepository {
    conn: DatabaseConnection,
}
impl SeaBrandRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl Repository<Brand, u32> for SeaBrandRepository {
    async fn save(&self, entity: Brand) -> Result<Brand, AppError> {
        let model: ActiveModel = entity.into();
        Ok(model.insert(&self.conn).await?.into())
    }

    async fn update(&self, entity: Brand) -> Result<Brand, AppError> {
        let id = entity
            .id()
            .ok_or(AppError::NotFound(String::from("Brand")))?;
        if Entity::find_by_id(id).one(&self.conn).await?.is_none() {
            return Err(AppError::NotFound(String::from("Brand not found!")));
        }

        let mut model: ActiveModel = entity.into();
        model.id = Set(id);
        let active = Entity::update(model).exec(&self.conn).await?;
        Ok(active.into())
    }
    async fn get_by_id(&self, id: u32) -> Result<Option<Brand>, AppError> {
        let model = Entity::find_by_id(id).one(&self.conn).await?;
        Ok(model.map(Into::into))
    }
    async fn get_all(&self) -> Result<Vec<Brand>, AppError> {
        let models = Entity::find().all(&self.conn).await?;
        let brands = models.into_iter().map(Into::into).collect();
        Ok(brands)
    }
    async fn delete_by_id(&self, id: u32) -> Result<(), AppError> {
        let result = Entity::delete_by_id(id).exec(&self.conn).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound(String::from("Brand not found!")));
        }
        Ok(())
    }
}
