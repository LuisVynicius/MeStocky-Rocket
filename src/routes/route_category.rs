use rocket::{State, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::category_dtos::CategoryDTO, guards::guard_user::Authentication, services::service_category};

#[get("/category")]
pub async fn route_category_get_all(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<Vec<CategoryDTO>> {

    let categories = service_category::get_all_categories(database).await;

    Json(categories)

}