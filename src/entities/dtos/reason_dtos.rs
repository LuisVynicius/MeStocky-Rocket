use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReasonDTO {
    id: u64,
    name: String,
}

impl ReasonDTO {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }

    pub fn get_id(&self) -> &u64 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReasonCreateDTO {
    name: String,
}

impl ReasonCreateDTO {
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
