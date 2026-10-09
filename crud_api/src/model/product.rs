use rust_decimal::Decimal;

use crate::model::{normalize_text, validations};

pub struct Product {
    id: Option<u32>,
    name: String,
    brand_id: Option<u32>,
    category_id: u32,
    cost: Decimal,
    price: Decimal,
    stock: u32,
    sku: String,
    description: Option<String>,
}
impl Product {
    pub fn new(
        id: Option<u32>,
        name: String,
        brand_id: Option<u32>,
        category_id: u32,
        cost: Decimal,
        price: Decimal,
        stock: u32,
        sku: String,
        description: Option<String>,
    ) -> Self {
        let id = validations::ids(id);
        let brand_id = validations::ids(brand_id);
        let description = validations::description(description);
        Self {
            id,
            name: normalize_text(&name),
            brand_id,
            category_id,
            cost,
            price,
            stock,
            sku: sku.split_whitespace().collect(),
            description: description.map(|d| normalize_text(&d)),
        }
    }

    pub fn id(&self) -> Option<u32> {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn brand_id(&self) -> Option<u32> {
        self.brand_id
    }
    pub fn cost(&self) -> Decimal {
        self.cost
    }
    pub fn price(&self) -> Decimal {
        self.price
    }
    pub fn category_id(&self) -> u32 {
        self.category_id
    }
    pub fn stock(&self) -> u32 {
        self.stock
    }
    pub fn sku(&self) -> &str {
        &self.sku
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
