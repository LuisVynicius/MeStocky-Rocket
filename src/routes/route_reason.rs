use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::reason_dtos::{ReasonCreateDTO, ReasonDTO}, guards::guard_user::{AuthenticationGuard, MannagerAuthenticationGuard, OperatorAuthenticationGuard}, routes::generic_functions::catch_backend_error, services::service_reason};

#[get("/reason")]
pub async fn route_reason_get_all(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _operator_authentication_guard: OperatorAuthenticationGuard,
) -> Result<Json<Vec<ReasonDTO>>, Custom<&'static str>> {

    let result = service_reason::get_all_reason(database).await;

    match result {
        Ok(reasons) => Ok(Json(reasons)),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[post("/reason", data="<reason_create_dto>")]
pub async fn route_reason_create(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
    reason_create_dto: Json<ReasonCreateDTO>
) -> Result<Status, Custom<&'static str>> {

    let result = service_reason::create_reason(database, reason_create_dto.0).await;

    match result {
        Ok(_) => Ok(Status::Created),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[put("/reason", data="<reason_update_dto>")]
pub async fn route_reason_update(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
    reason_update_dto: Json<ReasonDTO>
) -> Result<Status, Custom<&'static str>> {

    let result = service_reason::update_reason(database, reason_update_dto.0).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[delete("/reason/<reason_id>")]
pub async fn route_reason_delete(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
    reason_id: u64
) -> Result<Status, Custom<&'static str>> {

    let result = service_reason::delete_by_id(database, reason_id).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}