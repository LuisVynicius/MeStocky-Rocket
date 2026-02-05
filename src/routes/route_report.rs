use rocket::{State, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::report_dtos::ReportViewDTO, guards::guard_user::{AuthenticationGuard, ViewerAuthenticationGuard}, services::service_report};

#[get("/report")]
pub async fn route_report_get_all(
    database: &State<DatabaseConnection>,
    _authentication: AuthenticationGuard,
    _viewer_authentication_guard: ViewerAuthenticationGuard
) -> Json<Vec<ReportViewDTO>> {

    let reports = service_report::get_all_reports(database).await;

    Json(reports)

}