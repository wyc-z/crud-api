pub mod brand;
pub mod category;
pub mod product;
pub mod validations;

pub fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
