use rocket::{State, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::report_dtos::ReportViewDTO, guards::guard_user::{AuthenticationGuard, ViewerAuthenticationGuard}, routes::generic_functions::catch_backend_error, services::service_report};

#[get("/report")]
pub async fn route_report_get_all(
    database: &State<DatabaseConnection>,
    _authentication: AuthenticationGuard,
    _viewer_authentication_guard: ViewerAuthenticationGuard
) -> Result<Json<Vec<ReportViewDTO>>, Custom<&'static str>> {

    let result = service_report::get_all_reports(database).await;

    match result {
        Ok(reports) => Ok(Json(reports)),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}