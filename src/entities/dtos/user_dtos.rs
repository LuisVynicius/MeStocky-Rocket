use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserCreateDTO {
    username: String,
    password: String,
    email: String,
    role_id: u64
}

impl UserCreateDTO {

    pub fn new(username: String, password: String, email: String, role_id: u64) -> Self {
        Self {
            username,
            password,
            email,
            role_id
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn get_role_id(&self) -> &u64 {
        &self.role_id
    }

}

#[derive(Deserialize, Serialize)]
pub struct LoginDTO {
    email: String,
    password: String
}

impl LoginDTO {

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }
    
}