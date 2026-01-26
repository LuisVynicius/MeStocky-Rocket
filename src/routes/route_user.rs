use rocket::{State, http::Status, response::status::{self, Custom}, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::user_dtos::{AuthenticationDTO, LoginDTO, UserCreateDTO, UserCredentialsUpdateDTO, UserRoleUpdateDTO, UserSummaryForAdminDTO}, guards::guard_user::Authentication, services::service_user::{self}};

#[get("/user")]
pub async fn route_user_get_all(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<Vec<UserSummaryForAdminDTO>> {

    let users = service_user::get_all_users(database).await;

    Json(users)

}

#[post("/login", data="<login_dto>")]
pub async fn route_login(
    database: &State<DatabaseConnection>,
    login_dto: Json<LoginDTO>
) -> Result<status::Custom<Json<AuthenticationDTO>>, Status> {
    
    let result = service_user::login(database, login_dto.0).await;

    match result {
        Ok(token) => Ok(Custom(Status::Ok, Json(token))),
        Err(_) => Err(Status::Conflict)
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
    user_update_dto: Json<UserCredentialsUpdateDTO>
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