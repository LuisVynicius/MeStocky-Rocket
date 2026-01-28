use sea_orm::FromQueryResult;
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
    min_quantity: u64,
    category_id: u64
}

impl ProductCreateDTO {

    pub fn get_name(&self) -> &String {

        &self.name

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
    quantity: u64,
    reason_id: u64
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

    pub fn get_reason_id(&self) -> &u64 {

        &self.reason_id

    }

}

#[derive(FromQueryResult)]
pub struct ProductInformationsGetDTO {
    quantity: Option<u64>,
    total: Option<u64>,
    warnings: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct ProductInformationsViewDTO {
    quantity:u64,
    total: u64,
    warnings: u64,
}

impl From<ProductInformationsGetDTO> for ProductInformationsViewDTO {

    fn from(value: ProductInformationsGetDTO) -> Self {
        
        Self {
            quantity: value.quantity.unwrap_or(0),
            total: value.total.unwrap_or(0),
            warnings: value.warnings.unwrap_or(0)
        }

    }
    
}