use crate::{model::category::Category, service::error::AppError};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CategoryRequest {
    pub id: Option<u32>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct CategoryResponse {
    id: u32,
    pub name: String,
    pub description: Option<String>,
}

impl CategoryRequest {
    pub fn set_id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }
}

impl From<CategoryRequest> for Category {
    fn from(req: CategoryRequest) -> Self {
        Category::new(req.id, req.name, req.description)
    }
}

impl TryFrom<Category> for CategoryResponse {
    type Error = AppError;
    fn try_from(category: Category) -> Result<Self, Self::Error> {
        let id = category
            .id()
            .ok_or(AppError::InternalError(String::from("Category has no id!")))?;
        Ok(CategoryResponse {
            id,
            name: String::from(category.name()),
            description: category.description().map(str::to_owned),
        })
    }
}
