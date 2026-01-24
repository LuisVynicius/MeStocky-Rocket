use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoleCreateViewDTO {
    name: String
}

impl RoleCreateViewDTO {

    pub fn new(name: String) -> Self {
        
        Self {
            name
        }

    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

}