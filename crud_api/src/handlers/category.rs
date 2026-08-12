use crate::{
    dto::category_dto::{CategoryRequest, CategoryResponse},
    model::category::Category,
    service::{Service, error::AppError},
};
use actix_web::{HttpResponse, delete, get, post, put, web};
use std::sync::Arc;

pub struct AppState {
    service: Arc<dyn Service<Category, u32>>,
}

impl AppState {
    pub fn new(service: Arc<dyn Service<Category, u32>>) -> Self {
        Self { service }
    }
}

#[post("/categories")]
pub async fn create_category(
    state: web::Data<AppState>,
    category_req: web::Json<CategoryRequest>,
) -> Result<HttpResponse, AppError> {
    let category = category_req.into_inner().into();
    let saved = state.service.create(category).await?;
    let resp = CategoryResponse::try_from(saved)?;
    Ok(HttpResponse::Created().json(resp))
}

#[get("/categories")]
pub async fn get_categories(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let categories = state.service.get_all().await?;
    let resp: Vec<CategoryResponse> = categories
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/categories/{category_id}")]
pub async fn get_category(
    state: web::Data<AppState>,
    category_id: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    let id = category_id.into_inner();
    let category = state
        .service
        .get_by_id(id)
        .await?
        .ok_or(AppError::NotFound(String::from("Category")))?;
    let resp = CategoryResponse::try_from(category)?;
    Ok(HttpResponse::Ok().json(resp))
}

#[put("/categories/{category_id}")]
pub async fn update_category(
    state: web::Data<AppState>,
    category_id: web::Path<u32>,
    category_req: web::Json<CategoryRequest>,
) -> Result<HttpResponse, AppError> {
    let category = category_req
        .into_inner()
        .set_id(category_id.into_inner())
        .into();
    let updated = state.service.update(category).await?;
    let resp = CategoryResponse::try_from(updated)?;
    Ok(HttpResponse::Ok().json(resp))
}

#[delete("/categories/{category_id}")]
pub async fn delete_category(
    state: web::Data<AppState>,
    category_id: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    let id = category_id.into_inner();
    state.service.delete(id).await?;
    Ok(HttpResponse::NoContent().finish())
}
