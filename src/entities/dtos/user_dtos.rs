use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserSummaryForAdminDTO {
    username: String,
    email: String,
    role: String
}

impl UserSummaryForAdminDTO {

    pub fn new(username: String, email: String, role: String) -> Self {
        Self {
            username,
            email,
            role
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct UserCreateDTO {
    username: String,
    password: String,
    email: String,
    role_id: u64
}

impl UserCreateDTO {

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

#[derive(Serialize, Deserialize)]
pub struct UserRoleUpdateDTO {
    user_id: u64,
    role_id: u64
}

impl UserRoleUpdateDTO {
    
    pub fn get_user_id(&self) -> &u64 {
        &self.user_id
    }

    pub fn get_role_id(&self) -> &u64 {
        &self.role_id
    }

}

#[derive(Serialize, Deserialize)]
pub struct UserUpdateDTO {
    username: Option<String>,
    email: Option<String>,
}

impl UserUpdateDTO {
    
    pub fn get_username(&self) -> &Option<String> {
        &self.username
    }

    pub fn get_email(&self) -> &Option<String> {
        &self.email
    }

}

#[derive(Serialize, Deserialize)]
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