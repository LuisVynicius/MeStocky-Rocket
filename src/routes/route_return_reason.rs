use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::return_reason_dtos::{ReturnReasonCreateDTO, ReturnReasonDTO}, guards::guard_user::Authentication, services::service_return_reason};

#[get("/reason")]
pub async fn route_return_reason_get_all(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<Vec<ReturnReasonDTO>> {

    let reasons = service_return_reason::get_all_reason(database).await;

    Json(reasons)

}

#[post("/reason", data="<reason_create_dto>")]
pub async fn route_return_reason_create(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    reason_create_dto: Json<ReturnReasonCreateDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_return_reason::create_reason(database, reason_create_dto.0).await;

    match result {
        Ok(message) => Ok(Custom(Status::Created, message)),
        Err(_) => Err(Status::Conflict)
    }

}

#[put("/reason", data="<reason_update_dto>")]
pub async fn route_return_reason_update(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    reason_update_dto: Json<ReturnReasonDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_return_reason::update_reason(database, reason_update_dto.0).await;

    match result {

        Ok(message) => Ok(Custom(Status::Ok, message)),
        Err(_) => Err(Status::Conflict)

    }

}