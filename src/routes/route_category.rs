use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::category_dtos::{CategoryCreateDTO, CategoryDTO, CategoryViewDTO}, guards::guard_user::{AuthenticationGuard, MannagerAuthenticationGuard, OperatorAuthenticationGuard}, routes::generic_functions::catch_backend_error, services::service_category};

#[get("/category")]
pub async fn route_category_get_all(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _operator_authentication_guard: OperatorAuthenticationGuard
) -> Result<Json<Vec<CategoryDTO>>, Custom<&'static str>> {

    let result = service_category::get_all_categories(database).await;

    match result {
        Ok(categories) => Ok(Json(categories)),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[get("/category/admin")]
pub async fn route_category_get_all_admin(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard
) -> Result<Json<Vec<CategoryViewDTO>>, Custom<&'static str>> {

    let result = service_category::get_all_categories_admin(database).await;

    match result {
        Ok(categories) => Ok(Json(categories)),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[post("/category", data="<category_create_dto>")]
pub async fn route_category_create(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
    category_create_dto: Json<CategoryCreateDTO>
) -> Result<Status, Custom<&'static str>> {

    let result = service_category::create_category(database, category_create_dto.0).await;

    match result {
        Ok(_) => Ok(Status::Created),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[put("/category", data="<category_update_dto>")]
pub async fn route_category_update(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
    category_update_dto: Json<CategoryDTO>
) -> Result<Status, Custom<&'static str>> {

    let result = service_category::update_category(database, category_update_dto.0).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[delete("/category/<category_id>")]
pub async fn route_category_delete(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
    category_id: u64
) -> Result<Status, Custom<&'static str>> {

    let result = service_category::delete_by_id(database, category_id).await;

    match result {

        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error))

    }

}