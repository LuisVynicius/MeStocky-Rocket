use rocket::{State, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::report_dtos::ReportViewDTO, guards::guard_user::Authentication, services::service_report};

#[get("/report")]
pub async fn route_report_get_all(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<Vec<ReportViewDTO>> {

    let reports = service_report::get_all_reports(database).await;

    Json(reports)

}