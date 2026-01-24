use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::role_dtos::RoleCreateViewDTO, guards::guard_user::Authentication, services::service_role};

#[get("/role")]
pub async fn route_role_get_all(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<Vec<RoleCreateViewDTO>> {

    let roles = service_role::get_all_role(database).await;

    Json(roles)

}

#[post("/role", data="<role_create_dto>")]
pub async fn route_role_create(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    role_create_dto: Json<RoleCreateViewDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_role::create_role(database, role_create_dto.0).await;

    match result {
        Ok(message) => Ok(Custom(Status::Created, message)),
        Err(_) => Err(Status::Conflict)
    }

}
