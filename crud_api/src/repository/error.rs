use sea_orm::{DbErr, RuntimeErr};

use crate::service::error::AppError;

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        match &err {
            DbErr::Exec(RuntimeErr::SqlxError(e)) | DbErr::Query(RuntimeErr::SqlxError(e)) => {
                let msg = e.to_string().to_lowercase();
                if msg.contains("duplicate") {
                    AppError::Conflict(String::from("Duplicate value for a unique field!"))
                } else if msg.contains("foreign key constraint!") {
                    AppError::NotFound(String::from("Related entity not found!"))
                } else {
                    AppError::InternalError(err.to_string())
                }
            }
            _ => AppError::InternalError(err.to_string()),
        }
    }
}
