use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginDTO {
    username: String,
    password: String
}

impl LoginDTO {
    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }
}