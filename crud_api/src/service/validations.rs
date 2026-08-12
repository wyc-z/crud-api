use rust_decimal::Decimal;

use crate::service::error::AppError;

pub fn description(description: Option<&str>, is_product: bool) -> Result<(), AppError> {
    match description {
        Some(d) if !is_product && d.len() > 60 => Err(AppError::Invalid(String::from(
            "The description cannot contain more than 60 characters!",
        ))),
        Some(d) if is_product && d.len() > 120 => Err(AppError::Invalid(String::from(
            "The description cannot contain more than 120 characters!",
        ))),
        Some(_) => Ok(()),
        None => Ok(()),
    }
}
pub fn name(name: &str, is_product: bool) -> Result<(), AppError> {
    match name {
        name if name.trim().is_empty() || name.trim().len() < 3 => Err(AppError::Empty(
            String::from("The name must contain at least 3 characters!"),
        )),
        name if !is_product && name.trim().len() > 20 => Err(AppError::Empty(String::from(
            "The name cannot be longer than 20 characters!",
        ))),

        name if is_product && name.trim().len() > 50 => Err(AppError::Empty(String::from(
            "The name cannot be longer than 50 characters!",
        ))),
        _ => Ok(()),
    }
}
pub fn id(id: u32) -> Result<(), AppError> {
    if id == 0 {
        return Err(AppError::NotFound(String::from("The id was not found!")));
    }
    Ok(())
}
pub fn price(cost: Decimal, price: Decimal) -> Result<(), AppError> {
    if price <= cost {
        return Err(AppError::Invalid(String::from(
            "The price must be higher than the cost!",
        )));
    }
    Ok(())
}
pub fn sku(sku: &str) -> Result<(), AppError> {
    match sku {
        sku if sku.trim().is_empty() || sku.trim().len() < 3 => Err(AppError::Invalid(
            String::from("The sku must contain at least 3 characters!"),
        )),
        sku if sku.len() > 50 => Err(AppError::Invalid(String::from(
            "The sku cannot be longer than 50 characters!",
        ))),
        _ => Ok(()),
    }
}
