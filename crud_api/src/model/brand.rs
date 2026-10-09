use crate::model::{normalize_text, validations};

pub struct Brand {
    id: Option<u32>,
    name: String,
    description: Option<String>,
}
impl Brand {
    pub fn new(id: Option<u32>, name: String, description: Option<String>) -> Self {
        let id = validations::ids(id);
        let description = validations::description(description);
        Self {
            id,
            name: normalize_text(&name),
            description: description.map(|d| normalize_text(&d)),
        }
    }
    pub fn id(&self) -> Option<u32> {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
