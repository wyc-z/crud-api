use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    model::brand::Brand,
    repository::Repository,
    service::{Service, error::AppError, validations},
};

pub struct DefaultBrandService {
    repo: Arc<dyn Repository<Brand, u32>>,
}
impl DefaultBrandService {
    pub fn new(repo: Arc<dyn Repository<Brand, u32>>) -> Self {
        Self { repo }
    }
}
#[async_trait]
impl Service<Brand, u32> for DefaultBrandService {
    async fn create(&self, entity: Brand) -> Result<Brand, AppError> {
        validations::name(entity.name(), false)?;
        validations::description(entity.description(), false)?;
        self.repo.save(entity).await
    }
    async fn update(&self, entity: Brand) -> Result<Brand, AppError> {
        validations::name(entity.name(), false)?;
        validations::description(entity.description(), false)?;
        self.repo.update(entity).await
    }
    async fn delete(&self, id: u32) -> Result<(), AppError> {
        validations::id(id)?;
        self.repo.delete_by_id(id).await
    }
    async fn get_by_id(&self, id: u32) -> Result<Option<Brand>, AppError> {
        validations::id(id)?;
        self.repo.get_by_id(id).await
    }
    async fn get_all(&self) -> Result<Vec<Brand>, AppError> {
        self.repo.get_all().await
    }
}
