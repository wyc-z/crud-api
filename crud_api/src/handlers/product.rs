use std::sync::Arc;

use actix_web::{HttpResponse, delete, get, post, put, web};

use crate::{
    dto::product_dto::{ProductRequest, ProductResponse},
    service::{ProductService, error::AppError},
};

pub struct AppState {
    service: Arc<dyn ProductService>,
}
impl AppState {
    pub fn new(service: Arc<dyn ProductService>) -> Self {
        Self { service }
    }
}

#[post("/products")]
pub async fn create_product(
    state: web::Data<AppState>,
    product_req: web::Json<ProductRequest>,
) -> Result<HttpResponse, AppError> {
    let product = product_req.into_inner().into();
    let saved = state.service.create(product).await?;
    let resp = ProductResponse::try_from(saved)?;
    Ok(HttpResponse::Created().json(resp))
}

#[get("/products")]
pub async fn get_products(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let products = state.service.get_all().await?;
    let resp: Vec<ProductResponse> = products
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/products/category/{category_id}")]
pub async fn get_by_category(
    state: web::Data<AppState>,
    category_id: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    let id = category_id.into_inner();
    let products = state.service.get_by_category(id).await?;
    let resp: Vec<ProductResponse> = products
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/products/{product_id}")]
pub async fn get_by_id(
    state: web::Data<AppState>,
    product_id: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    let id = product_id.into_inner();
    let product = state
        .service
        .get_by_id(id)
        .await?
        .ok_or(AppError::NotFound(String::from("Product")))?;
    let resp = ProductResponse::try_from(product)?;
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/products/sku/{product_sku}")]
pub async fn get_by_sku(
    state: web::Data<AppState>,
    product_sku: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let sku = product_sku.into_inner();
    let product = state
        .service
        .get_by_sku(&sku)
        .await?
        .ok_or(AppError::NotFound(String::from("Product")))?;
    let resp = ProductResponse::try_from(product)?;
    Ok(HttpResponse::Ok().json(resp))
}

#[put("/products/{product_id}")]
pub async fn update_product(
    state: web::Data<AppState>,
    product_id: web::Path<u32>,
    product_req: web::Json<ProductRequest>,
) -> Result<HttpResponse, AppError> {
    let product = product_req
        .into_inner()
        .set_id(product_id.into_inner())
        .into();
    let updated = state.service.update(product).await?;
    let resp = ProductResponse::try_from(updated)?;
    Ok(HttpResponse::Ok().json(resp))
}
#[delete("/products/{product_id}")]
pub async fn delete_by_id(
    state: web::Data<AppState>,
    product_id: web::Path<u32>,
) -> Result<HttpResponse, AppError> {
    let id = product_id.into_inner();
    state.service.delete(id).await?;
    Ok(HttpResponse::NoContent().finish())
}
