use crate::{
    dto::brand_dto::{BrandRequest, BrandResponse},
    model::brand::Brand,
    service::{Service, error::AppError},
};
use actix_web::{
    HttpResponse, delete, get, post, put,
    web::{self, Json},
};
use std::sync::Arc;

pub struct AppState {
    service: Arc<dyn Service<Brand, u32>>,
}

impl AppState {
    pub fn new(service: Arc<dyn Service<Brand, u32>>) -> Self {
        Self { service }
    }
}

#[post("/brands")]
pub async fn create_brand(
    state: web::Data<AppState>,
    brand_req: Json<BrandRequest>,
) -> Result<HttpResponse, AppError> {
    let brand = brand_req.into_inner().into();
    let saved = state.service.create(brand).await?;
    let resp = BrandResponse::try_from(saved)?;
    Ok(HttpResponse::Created().json(resp))
}

#[get("/brands")]
pub async fn get_brands(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let brand = state.service.get_all().await?;
    let resp: Vec<BrandResponse> = brand
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/brands/{brand_id}")]
pub async fn get_by_id(
    state: web::Data<AppState>,
    brand_id: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    let id = brand_id.into_inner();
    let brand = state
        .service
        .get_by_id(id)
        .await?
        .ok_or(AppError::NotFound(String::from("Brand")))?;
    let resp = BrandResponse::try_from(brand)?;
    Ok(HttpResponse::Ok().json(resp))
}

#[put("/brands/{brand_id}")]
pub async fn update_brand(
    state: web::Data<AppState>,
    brand_id: web::Path<u32>,
    brand_req: Json<BrandRequest>,
) -> Result<HttpResponse, AppError> {
    let brand = brand_req.into_inner().set_id(brand_id.into_inner()).into();
    let updated = state.service.update(brand).await?;
    let resp = BrandResponse::try_from(updated)?;
    Ok(HttpResponse::Ok().json(resp))
}

#[delete("/brands/{brand_id}")]
pub async fn delete_brand(
    state: web::Data<AppState>,
    brand_id: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    let id = brand_id.into_inner();
    state.service.delete(id).await?;
    Ok(HttpResponse::NoContent().finish())
}
