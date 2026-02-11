use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{
    entities::dtos::report_dtos::{ReportUpdateDTO, ReportViewDTO},
    guards::guard_user::{AuthenticationGuard, OperatorAuthenticationGuard, ViewerAuthenticationGuard},
    routes::generic_functions::catch_backend_error,
    services::service_report,
};

#[get("/report")]
pub async fn route_report_get_all(
    database: &State<DatabaseConnection>,
    _authentication: AuthenticationGuard,
    _viewer_authentication_guard: ViewerAuthenticationGuard,
) -> Result<Json<Vec<ReportViewDTO>>, Custom<&'static str>> {
    let result = service_report::get_all_reports(database).await;

    match result {
        Ok(reports) => Ok(Json(reports)),
        Err(backend_error) => Err(catch_backend_error(backend_error)),
    }
}

#[put("/report", data="<report_update_dto>")]
pub async fn route_report_update(
    database: &State<DatabaseConnection>,
    _authentication: AuthenticationGuard,
    _operator_authentication_guard: OperatorAuthenticationGuard,
    report_update_dto: Json<ReportUpdateDTO>
) -> Result<Status, Custom<&'static str>> {
    let result = service_report::update_report(database, report_update_dto.0).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error)),
    }
}