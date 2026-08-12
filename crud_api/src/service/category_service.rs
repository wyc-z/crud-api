use std::sync::Arc;

use async_trait::async_trait;

use crate::repository::Repository;
use crate::service::validations;
use crate::{
    model::category::Category,
    service::{Service, error::AppError},
};
pub struct DefaultCategoryService {
    repo: Arc<dyn Repository<Category, u32>>,
}
impl DefaultCategoryService {
    pub fn new(repo: Arc<dyn Repository<Category, u32>>) -> Self {
        Self { repo }
    }
}
#[async_trait]
impl Service<Category, u32> for DefaultCategoryService {
    async fn create(&self, entity: Category) -> Result<Category, AppError> {
        validations::name(entity.name(), false)?;
        validations::description(entity.description(), false)?;
        self.repo.save(entity).await
    }
    async fn update(&self, entity: Category) -> Result<Category, AppError> {
        validations::name(entity.name(), false)?;
        validations::description(entity.description(), false)?;
        self.repo.update(entity).await
    }
    async fn delete(&self, id: u32) -> Result<(), AppError> {
        validations::id(id)?;
        self.repo.delete_by_id(id).await
    }
    async fn get_by_id(&self, id: u32) -> Result<Option<Category>, AppError> {
        validations::id(id)?;
        self.repo.get_by_id(id).await
    }
    async fn get_all(&self) -> Result<Vec<Category>, AppError> {
        self.repo.get_all().await
    }
}
