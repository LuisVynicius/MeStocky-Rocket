use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::entities::enums::user_enums::UserRole;

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct UserSummaryForAdminQueryDTO {
    id: u64,
    username: String,
    email: String,
    role: u8
}

#[derive(Serialize, Deserialize)]
pub struct UserSummaryForAdminDTO {
    id: u64,
    username: String,
    email: String,
    role: String
}

impl From<UserSummaryForAdminQueryDTO> for UserSummaryForAdminDTO {

    fn from(value: UserSummaryForAdminQueryDTO) -> Self {

        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            role: UserRole::code_to_string(value.role)
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

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_email(&self) -> &str {
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
pub struct UserCredentialsUpdateDTO {
    old_password: String,
    new_password: String,
}

impl UserCredentialsUpdateDTO {
    
    pub fn get_old_password(&self) -> &str {

        &self.old_password

    }

    pub fn get_new_password(&self) -> &str {

        &self.new_password

    }

}

#[derive(Serialize, Deserialize)]
pub struct UserInformationsUpdateDTO {
    username: String,
    email: String,
}

impl UserInformationsUpdateDTO {
    
    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    email: String,
    password: String
}

impl LoginDTO {

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password(&self) -> &str {
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