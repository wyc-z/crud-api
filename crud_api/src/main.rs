use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use crud_api::{
    handlers::{brand, category, product},
    model::{brand::Brand, category::Category},
    repository::{
        ProductRepository, Repository, brand_repository::SeaBrandRepository,
        category_repository::SeaCategoryRepository, connection::connection,
        product_repository::SeaProductRepository,
    },
    service::{
        ProductService, Service, brand_service::DefaultBrandService,
        category_service::DefaultCategoryService, product_service::DefaultProductService,
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = connection().await?;

    HttpServer::new(move || {
        let product_repo =
            Arc::new(SeaProductRepository::new(conn.clone())) as Arc<dyn ProductRepository>;
        let product_serv =
            Arc::new(DefaultProductService::new(product_repo)) as Arc<dyn ProductService>;
        let product_state = product::AppState::new(product_serv);
        let category_repo = Arc::new(SeaCategoryRepository::new(conn.clone()))
            as Arc<dyn Repository<Category, u32>>;
        let category_serv =
            Arc::new(DefaultCategoryService::new(category_repo)) as Arc<dyn Service<Category, u32>>;
        let category_state = category::AppState::new(category_serv);
        let brand_repo =
            Arc::new(SeaBrandRepository::new(conn.clone())) as Arc<dyn Repository<Brand, u32>>;
        let brand_serv =
            Arc::new(DefaultBrandService::new(brand_repo)) as Arc<dyn Service<Brand, u32>>;
        let brand_state = brand::AppState::new(brand_serv);
        App::new()
            .app_data(web::Data::new(product_state))
            .app_data(web::Data::new(category_state))
            .app_data(web::Data::new(brand_state))
            .service(product::create_product)
            .service(product::get_products)
            .service(product::get_by_category)
            .service(product::get_by_sku)
            .service(product::get_by_id)
            .service(product::update_product)
            .service(product::delete_by_id)
            .service(category::create_category)
            .service(category::get_categories)
            .service(category::get_category)
            .service(category::update_category)
            .service(category::delete_category)
            .service(brand::create_brand)
            .service(brand::get_brands)
            .service(brand::get_by_id)
            .service(brand::update_brand)
            .service(brand::delete_brand)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
