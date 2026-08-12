use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    model::product::Product,
    repository::ProductRepository,
    service::{ProductService, Service, error::AppError, validations},
};

pub struct DefaultProductService {
    repo: Arc<dyn ProductRepository>,
}
impl DefaultProductService {
    pub fn new(repo: Arc<dyn ProductRepository>) -> Self {
        Self { repo }
    }
}
#[async_trait]
impl Service<Product, u32> for DefaultProductService {
    async fn create(&self, entity: Product) -> Result<Product, AppError> {
        validations::name(entity.name(), true)?;
        validations::description(entity.description(), true)?;
        validations::price(entity.cost(), entity.price())?;
        validations::sku(entity.sku())?;
        self.repo.save(entity).await
    }
    async fn update(&self, entity: Product) -> Result<Product, AppError> {
        let id = entity
            .id()
            .ok_or(AppError::Invalid(String::from("The product has no id!")))?;
        validations::name(entity.name(), true)?;
        validations::id(id)?;
        validations::description(entity.description(), true)?;
        validations::price(entity.cost(), entity.price())?;
        validations::sku(entity.sku())?;
        self.repo.update(entity).await
    }
    async fn delete(&self, id: u32) -> Result<(), AppError> {
        validations::id(id)?;
        self.repo.delete_by_id(id).await
    }
    async fn get_by_id(&self, id: u32) -> Result<Option<Product>, AppError> {
        validations::id(id)?;
        self.repo.get_by_id(id).await
    }
    async fn get_all(&self) -> Result<Vec<Product>, AppError> {
        self.repo.get_all().await
    }
}
#[async_trait]
impl ProductService for DefaultProductService {
    async fn get_by_category(&self, category_id: u32) -> Result<Vec<Product>, AppError> {
        self.repo.get_by_category(category_id).await
    }
    async fn get_by_sku(&self, sku: &str) -> Result<Option<Product>, AppError> {
        self.repo.get_by_sku(sku).await
    }
}
