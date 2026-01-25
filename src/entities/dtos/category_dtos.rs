use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CategoryDTO {
    id: u64,
    name: String
}

impl CategoryDTO {

    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name
        }
    }

}


#[derive(Serialize, Deserialize)]
pub struct CategoryCreateDTO {
    name: String
}

impl CategoryCreateDTO {

    pub fn get_name(&self) -> &String {
        &self.name
    }

}
