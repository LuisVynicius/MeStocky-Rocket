use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::role_dtos::{RoleCreateDTO, RoleDTO}, guards::guard_user::Authentication, services::service_role};

#[get("/role")]
pub async fn route_role_get_all(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<Vec<RoleDTO>> {

    let roles = service_role::get_all_roles(database).await;

    Json(roles)

}

#[post("/role", data="<role_create_dto>")]
pub async fn route_role_create(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    role_create_dto: Json<RoleCreateDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_role::create_role(database, role_create_dto.0).await;

    match result {
        Ok(message) => Ok(Custom(Status::Created, message)),
        Err(_) => Err(Status::Conflict)
    }

}

#[put("/role", data="<role_update_dto>")]
pub async fn route_role_update(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    role_update_dto: Json<RoleDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_role::update_role(database, role_update_dto.0).await;

    match result {

        Ok(message) => Ok(Custom(Status::Ok, message)),
        Err(_) => Err(Status::Conflict)

    }

}