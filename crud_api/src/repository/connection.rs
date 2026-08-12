use sea_orm::{Database, DatabaseConnection};

use crate::service::error::AppError;

pub async fn connection() -> Result<DatabaseConnection, AppError> {
    dotenvy::dotenv().ok();
    let path = std::env::var("DATABASE_URL")
        .map_err(|_| AppError::InternalError(String::from("DATABASE_URL not set!")))?;
    let database = Database::connect(&path).await?;
    Ok(database)
}
