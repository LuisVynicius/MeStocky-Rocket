use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::category_dtos::{CategoryCreateDTO, CategoryDTO}, guards::guard_user::Authentication, services::service_category};

#[get("/category")]
pub async fn route_category_get_all(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<Vec<CategoryDTO>> {

    let categories = service_category::get_all_categories(database).await;

    Json(categories)

}

#[post("/category", data="<category_create_dto>")]
pub async fn route_category_create(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    category_create_dto: Json<CategoryCreateDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_category::create_category(database, category_create_dto.0).await;

    match result {
        Ok(message) => Ok(Custom(Status::Created, message)),
        Err(_) => Err(Status::Conflict)
    }

}

#[put("/category", data="<category_update_dto>")]
pub async fn route_category_update(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    category_update_dto: Json<CategoryDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_category::update_category(database, category_update_dto.0).await;

    match result {

        Ok(message) => Ok(Custom(Status::Ok, message)),
        Err(_) => Err(Status::Conflict)

    }

}