use serde::{Deserialize, Serialize};

use crate::{model::brand::Brand, service::error::AppError};

#[derive(Deserialize)]
pub struct BrandRequest {
    pub id: Option<u32>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct BrandResponse {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

impl BrandRequest {
    pub fn set_id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }
}

impl From<BrandRequest> for Brand {
    fn from(req: BrandRequest) -> Self {
        Brand::new(req.id, req.name, req.description)
    }
}

impl TryFrom<Brand> for BrandResponse {
    type Error = AppError;
    fn try_from(brand: Brand) -> Result<Self, Self::Error> {
        let id = brand
            .id()
            .ok_or(AppError::InternalError(String::from("Brand has no id!")))?;

        Ok(BrandResponse {
            id,
            name: String::from(brand.name()),
            description: brand.description().map(str::to_owned),
        })
    }
}
