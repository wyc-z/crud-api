use actix_web::{HttpResponse, ResponseError, body::BoxBody, http::StatusCode};
use sea_orm::sea_query::value::prelude::serde_json;

use crate::service::error::AppError;

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = match self {
            AppError::InternalError(msg) => {
                eprintln!("[internal] {msg}");
                serde_json::json!({ "error": "internal server error" })
            }
            other => serde_json::json!({ "error": other.to_string() }),
        };
        HttpResponse::build(self.status_code()).json(body)
    }
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Empty(_) | AppError::Invalid(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
