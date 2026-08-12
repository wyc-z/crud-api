use super::{ProductRepository, Repository};
use crate::{model::product::Product, service::error::AppError};
use async_trait::async_trait;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    QueryOrder,
    prelude::*,
};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub brand_id: Option<u32>,
    #[sea_orm(belongs_to, from = "brand_id", to = "id")]
    pub brand: BelongsTo<Option<super::brand_repository::Entity>>,
    pub category_id: u32,
    #[sea_orm(belongs_to, from = "category_id", to = "id")]
    pub category: BelongsTo<super::category_repository::Entity>,
    pub cost: Decimal,
    pub price: Decimal,
    pub stock: u32,
    #[sea_orm(unique)]
    pub sku: String,
    pub description: Option<String>,
}
impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for Product {
    fn from(model: Model) -> Self {
        Product::new(
            Some(model.id),
            model.name,
            model.brand_id,
            model.category_id,
            model.cost,
            model.price,
            model.stock,
            model.sku,
            model.description,
        )
    }
}
impl From<Product> for ActiveModel {
    fn from(product: Product) -> Self {
        ActiveModel {
            id: NotSet,
            name: Set(String::from(product.name())),
            brand_id: Set(product.brand_id()),
            category_id: Set(product.category_id()),
            cost: Set(product.cost()),
            price: Set(product.price()),
            stock: Set(product.stock()),
            sku: Set(String::from(product.sku())),
            description: Set(product.description().map(str::to_owned)),
        }
    }
}

pub struct SeaProductRepository {
    conn: DatabaseConnection,
}
impl SeaProductRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl Repository<Product, u32> for SeaProductRepository {
    async fn save(&self, entity: Product) -> Result<Product, AppError> {
        let model: ActiveModel = entity.into();
        Ok(model.insert(&self.conn).await?.into())
    }
    async fn update(&self, entity: Product) -> Result<Product, AppError> {
        let id = entity
            .id()
            .ok_or(AppError::NotFound(String::from("Product")))?;
        if Entity::find_by_id(id).one(&self.conn).await?.is_none() {
            return Err(AppError::NotFound(String::from("Product not found!")));
        }
        let mut model: ActiveModel = entity.into();
        model.id = Set(id);
        let active = Entity::update(model).exec(&self.conn).await?;
        Ok(active.into())
    }
    async fn get_by_id(&self, id: u32) -> Result<Option<Product>, AppError> {
        let model = Entity::find_by_id(id).one(&self.conn).await?;
        Ok(model.map(Into::into))
    }
    async fn get_all(&self) -> Result<Vec<Product>, AppError> {
        let models = Entity::find().all(&self.conn).await?;
        let products = models.into_iter().map(Into::into).collect();
        Ok(products)
    }
    async fn delete_by_id(&self, id: u32) -> Result<(), AppError> {
        let result = Entity::delete_by_id(id).exec(&self.conn).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound(String::from("Product not found!")));
        }
        Ok(())
    }
}
#[async_trait]
impl ProductRepository for SeaProductRepository {
    async fn get_by_category(&self, category_id: u32) -> Result<Vec<Product>, AppError> {
        let model = Entity::find()
            .filter(self::Column::CategoryId.eq(category_id))
            .order_by_asc(Column::CategoryId)
            .all(&self.conn)
            .await?;
        let products = model.into_iter().map(Into::into).collect();
        Ok(products)
    }
    async fn get_by_sku(&self, sku: &str) -> Result<Option<Product>, AppError> {
        let product = Entity::find_by_sku(sku).one(&self.conn).await?;
        Ok(product.map(Into::into))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_product_to_model() {
        use super::ActiveModel;
        use crate::model::product::Product;
        use rust_decimal_macros::dec;
        use sea_orm::ActiveValue::Set;

        let product = Product::new(
            None,
            String::from("Mi 13 Ultra"),
            Some(2),
            3,
            dec!(890.87),
            dec!(1300.97),
            3,
            String::from("088383202921"),
            None,
        );
        let model: ActiveModel = product.into();

        assert_eq!(model.name, Set(String::from("Mi 13 Ultra")));
        assert_eq!(model.sku, Set(String::from("088383202921")));
        assert_eq!(model.brand_id, Set(Some(2)));
        assert_eq!(model.price, Set(dec!(1300.97)));
        assert_eq!(model.description, Set(None));
    }

    #[test]
    fn test_model_to_product() {
        use super::Model;
        use crate::model::product::Product;
        use rust_decimal_macros::dec;

        let model = Model {
            id: 21,
            name: String::from("Galaxy s26 Ultra"),
            brand_id: Some(7),
            category_id: 3,
            cost: dec!(1100.89),
            price: dec!(1700.99),
            stock: 5,
            sku: String::from("1121123671"),
            description: None,
        };
        let product: Product = model.into();

        assert_eq!(product.name(), "Galaxy s26 Ultra");
        assert_eq!(product.sku(), "1121123671");
        assert_eq!(product.brand_id(), Some(7));
        assert_eq!(product.price(), dec!(1700.99));
        assert_eq!(product.description(), None);
    }
}
