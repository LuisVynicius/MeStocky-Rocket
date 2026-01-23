use rocket::{State, http::Status, response::status::{self, Custom}, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{configs::config_jwt::generate_token, entities::dtos::user_dtos::{LoginDTO, UserCreateDTO}, services::service_user::{self, find_user_by_email}};

#[post("/login", data="<logindto>")]
pub async fn route_login(database: &State<DatabaseConnection>, logindto: Json<LoginDTO>) -> Result<status::Custom<String>, Status> {
    
    let user = find_user_by_email(database, logindto.get_email().clone()).await;

    match user {
        Ok(user) => {
            if user.1.password.eq(logindto.get_password()) {
                let token = generate_token(user.1.email.clone());

                return Ok(Custom(Status::Ok, token.unwrap()));
            }
            Err(Status::InternalServerError)
        },
        Err(status) => Err(status)
    }

}

#[post("/user", data="<user_dto>")]
pub async fn route_user_create(database: &State<DatabaseConnection>, user_dto: Json<UserCreateDTO>) {
    
    service_user::create_user(database, user_dto.0).await;

}