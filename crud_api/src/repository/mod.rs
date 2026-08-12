pub mod brand_repository;
pub mod category_repository;
pub mod connection;
pub mod error;
pub mod product_repository;

use crate::{model::product::Product, service::error::AppError};
use async_trait::async_trait;

#[async_trait]
pub trait Repository<T, Id>: Send + Sync {
    async fn save(&self, entity: T) -> Result<T, AppError>;
    async fn update(&self, entity: T) -> Result<T, AppError>;
    async fn get_by_id(&self, id: Id) -> Result<Option<T>, AppError>;
    async fn get_all(&self) -> Result<Vec<T>, AppError>;
    async fn delete_by_id(&self, id: Id) -> Result<(), AppError>;
}
#[async_trait]
pub trait ProductRepository: Repository<Product, u32> {
    async fn get_by_category(&self, category_id: u32) -> Result<Vec<Product>, AppError>;
    async fn get_by_sku(&self, sku: &str) -> Result<Option<Product>, AppError>;
}
