use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserSummaryForAdminDTO {
    id: u64,
    username: String,
    email: String,
    role: String
}

impl UserSummaryForAdminDTO {

    pub fn new(id: u64, username: String, email: String, role: String) -> Self {
        Self {
            id,
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
    role: u8
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

    pub fn get_role(&self) -> &u8 {
        &self.role
    }

}

#[derive(Serialize, Deserialize)]
pub struct UserRoleUpdateDTO {
    user_id: u64,
    role: u8
}

impl UserRoleUpdateDTO {
    
    pub fn get_user_id(&self) -> &u64 {
        &self.user_id
    }

    pub fn get_role(&self) -> &u8 {
        &self.role
    }

}

#[derive(Serialize, Deserialize)]
pub struct UserInformationsUpdateDTO {
    username: Option<String>,
    email: Option<String>,
}

impl UserInformationsUpdateDTO {
    
    pub fn get_username(&self) -> &Option<String> {
        &self.username
    }

    pub fn set_username(&mut self, username: Option<String> ) {
        self.username = username;
    }

    pub fn get_email(&self) -> &Option<String> {
        &self.email
    }

    pub fn set_email(&mut self, email: Option<String> ) {
        self.email = email;
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

#[derive(Serialize, Deserialize)]
pub struct ValidedTokenDTO {
    valided: bool
}

impl ValidedTokenDTO {

    pub fn new(valided: bool) -> Self {

        Self {
            valided
        }

    }

    pub fn get_valided(&self) -> &bool {

        &self.valided

    } 

}

#[derive(Serialize, Deserialize)]
pub struct AuthenticationDTO {
    token: String,
    role: u8,
    username: String,
    rolename: String
}

impl AuthenticationDTO {

    pub fn new(
        token: String,
        role: u8,
        username: String,
        role_name: String
    ) -> Self {

        Self {
            token,
            role,
            username,
            rolename: role_name
        }

    }

}