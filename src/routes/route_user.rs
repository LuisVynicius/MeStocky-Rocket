use rocket::{State, http::Status, response::status::{self, Custom}, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{configs::config_jwt::generate_token, entities::dtos::user_dtos::{LoginDTO, UserCreateDTO, UserRoleUpdateDTO, UserSummaryForAdminDTO, UserUpdateDTO}, guards::guard_user::Authentication, services::service_user::{self, find_user_by_email}};

#[get("/user")]
pub async fn route_user_get_all(
    database: &State<DatabaseConnection>,
    authentication: Authentication
) -> Json<Vec<UserSummaryForAdminDTO>> {

    let users = service_user::get_all_users(database).await;

    Json(users)

}

#[post("/login", data="<login_dto>")]
pub async fn route_login(
    database: &State<DatabaseConnection>,
    login_dto: Json<LoginDTO>
) -> Result<status::Custom<String>, Status> {
    
    let user = find_user_by_email(database, login_dto.get_email().clone()).await;

    match user {
        Some(user) => {
            if user.password.eq(login_dto.get_password()) {
                let token = generate_token(user.email.clone());

                return Ok(Custom(Status::Ok, token.unwrap()));
            }
            Err(Status::InternalServerError)
        },
        None => Err(Status::NotFound)
    }

}

#[post("/user", data="<user_create_dto>")]
pub async fn route_user_create(
    database: &State<DatabaseConnection>,
    user_create_dto: Json<UserCreateDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_user::create_user(database, user_create_dto.0).await;

    match result {
        Ok(message) => Ok(Custom(Status::Created, message)),
        Err(_) => Err(Status::Conflict)
    }

}

#[put("/user", data="<user_update_dto>")]
pub async fn route_user_update(
    database: &State<DatabaseConnection>,
    authentication: Authentication,
    user_update_dto: Json<UserUpdateDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_user::update_user(database, user_update_dto.0, authentication).await;

    match result {
        Ok(message) => Ok(Custom(Status::Ok, message)),
        Err(_) => Err(Status::Forbidden)
    }

}

#[put("/user/role", data="<user_role_update_dto>")]
pub async fn route_user_role_update(
    database: &State<DatabaseConnection>,
    authentication: Authentication,
    user_role_update_dto: Json<UserRoleUpdateDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_user::switch_role(database, user_role_update_dto.0, authentication).await;

    match result {
        Ok(message) => Ok(Custom(Status::Ok, message)),
        Err(_) => Err(Status::Forbidden)
    }

}