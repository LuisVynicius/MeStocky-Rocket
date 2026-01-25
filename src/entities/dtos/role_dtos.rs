use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoleCreateDTO {
    name: String
}

impl RoleCreateDTO {

    pub fn new(name: String) -> Self {
        
        Self {
            name
        }

    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

}

#[derive(Serialize, Deserialize)]
pub struct RoleDTO {
    id: u64,
    name: String
}

impl RoleDTO {

    pub fn new(id: u64, name: String) -> Self {
        
        Self {
            id,
            name
        }

    }

    pub fn get_id(&self) -> &u64 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name    }

}