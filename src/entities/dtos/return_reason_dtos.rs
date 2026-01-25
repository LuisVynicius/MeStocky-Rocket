use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ReturnReasonDTO {
    id: u64,
    name: String
}

impl ReturnReasonDTO {

    pub fn new(
        id: u64,
        name: String
    ) -> Self {

        Self {
            id,
            name
        }

    }

    pub fn get_id(&self) -> &u64 {

        &self.id

    }

    pub fn get_name(&self) -> &String {

        &self.name

    }

}

#[derive(Serialize, Deserialize)]
pub struct ReturnReasonCreateDTO {
    name: String
}

impl ReturnReasonCreateDTO {

    pub fn get_name(&self) -> &String {

        &self.name

    }
    
}


