use crate::{model::product::Product, service::error::AppError};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProductRequest {
    pub id: Option<u32>,
    pub name: String,
    pub brand_id: Option<u32>,
    pub category_id: u32,
    pub cost: Decimal,
    pub price: Decimal,
    pub stock: u32,
    pub sku: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct ProductResponse {
    pub id: u32,
    pub name: String,
    pub brand_id: Option<u32>,
    pub category_id: u32,
    pub cost: Decimal,
    pub price: Decimal,
    pub stock: u32,
    pub sku: String,
    pub description: Option<String>,
}

impl ProductRequest {
    pub fn set_id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }
}

impl From<ProductRequest> for Product {
    fn from(req: ProductRequest) -> Self {
        Product::new(
            req.id,
            req.name,
            req.brand_id,
            req.category_id,
            req.cost,
            req.price,
            req.stock,
            req.sku,
            req.description,
        )
    }
}

impl TryFrom<Product> for ProductResponse {
    type Error = AppError;
    fn try_from(product: Product) -> Result<Self, Self::Error> {
        let id = product
            .id()
            .ok_or(AppError::InternalError(String::from("Product has no id!")))?;
        Ok(ProductResponse {
            id,
            name: String::from(product.name()),
            brand_id: product.brand_id(),
            category_id: product.category_id(),
            cost: product.cost(),
            price: product.price(),
            stock: product.stock(),
            sku: String::from(product.sku()),
            description: product.description().map(str::to_owned),
        })
    }
}
