use async_trait::async_trait;

use crate::{model::product::Product, service::error::AppError};
pub mod brand_service;
pub mod category_service;
pub mod error;
pub mod product_service;
pub mod validations;

#[async_trait]
pub trait Service<T, Id> {
    async fn create(&self, entity: T) -> Result<T, AppError>;
    async fn update(&self, entity: T) -> Result<T, AppError>;
    async fn delete(&self, id: Id) -> Result<(), AppError>;
    async fn get_by_id(&self, id: Id) -> Result<Option<T>, AppError>;
    async fn get_all(&self) -> Result<Vec<T>, AppError>;
}
#[async_trait]
pub trait ProductService: Service<Product, u32> {
    async fn get_by_category(&self, category_id: u32) -> Result<Vec<Product>, AppError>;
    async fn get_by_sku(&self, sku: &str) -> Result<Option<Product>, AppError>;
}
