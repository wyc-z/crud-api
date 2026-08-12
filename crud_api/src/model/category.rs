use crate::model::validations;

pub struct Category {
    id: Option<u32>,
    name: String,
    description: Option<String>,
}
impl Category {
    pub fn new(id: Option<u32>, name: String, description: Option<String>) -> Self {
        let id = validations::ids(id);
        let description = validations::description(description);
        Self {
            id,
            name,
            description,
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
