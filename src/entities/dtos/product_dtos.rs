use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProductDTO {
    id: u64,
    name: String,
    quantity: u64,
    min_quantity: u64,
    category_id: u64
}

impl ProductDTO {

    pub fn get_id(&self) -> &u64 {

        &self.id

    }

    pub fn get_name(&self) -> &String {

        &self.name

    }

    pub fn get_quantity(&self) -> &u64 {

        &self.quantity

    }

    pub fn get_min_quantity(&self) -> &u64 {

        &self.min_quantity

    }

    pub fn get_category_id(&self) -> &u64 {

        &self.category_id

    }

}

#[derive(Serialize, Deserialize)]
pub struct ProductCreateDTO {
    name: String,
    quantity: u64,
    min_quantity: u64,
    category_id: u64
}

impl ProductCreateDTO {

    pub fn get_name(&self) -> &String {

        &self.name

    }

    pub fn get_quantity(&self) -> &u64 {

        &self.quantity

    }

    pub fn get_min_quantity(&self) -> &u64 {

        &self.min_quantity

    }

    pub fn get_category_id(&self) -> &u64 {

        &self.category_id

    }

}

#[derive(Serialize, Deserialize)]
pub struct ProductViewDTO {
    id: u64,
    name: String,
    quantity: u64,
    min_quantity: u64,
    category: String
}

impl ProductViewDTO {

    pub fn new(
        id: u64,
        name: String,
        quantity: u64,
        min_quantity: u64,
        category: String
    ) -> Self {

        Self {
            id,
            name,
            quantity,
            min_quantity,
            category
        }

    }

}

#[derive(Serialize, Deserialize)]
pub struct ProductChangeQuantityDTO {
    id: u64,
    change_type: bool,
    quantity: u64
}

impl ProductChangeQuantityDTO {

    pub fn get_id(&self) -> &u64 {

        &self.id

    }

    pub fn get_change_type(&self) -> &bool {

        &self.change_type

    }

    pub fn get_quantity(&self) -> &u64 {

        &self.quantity

    }

}