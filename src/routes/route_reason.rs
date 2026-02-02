use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::reason_dtos::{ReasonCreateDTO, ReasonDTO}, guards::guard_user::Authentication, services::service_reason};

#[get("/reason")]
pub async fn route_return_reason_get_all(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<Vec<ReasonDTO>> {

    let reasons = service_reason::get_all_reason(database).await;

    Json(reasons)

}

#[post("/reason", data="<reason_create_dto>")]
pub async fn route_return_reason_create(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    reason_create_dto: Json<ReasonCreateDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_reason::create_reason(database, reason_create_dto.0).await;

    match result {
        Ok(message) => Ok(Custom(Status::Created, message)),
        Err(_) => Err(Status::Conflict)
    }

}

#[put("/reason", data="<reason_update_dto>")]
pub async fn route_return_reason_update(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    reason_update_dto: Json<ReasonDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_reason::update_reason(database, reason_update_dto.0).await;

    match result {

        Ok(message) => Ok(Custom(Status::Ok, message)),
        Err(_) => Err(Status::Conflict)

    }

}