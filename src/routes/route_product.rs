use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::product_dtos::{ProductChangeQuantityDTO, ProductCreateDTO, ProductInformationsViewDTO, ProductUpdateDTO, ProductViewDTO}, guards::guard_user::{AuthenticationGuard, OperatorAuthenticationGuard, ViewerAuthenticationGuard}, routes::generic_functions::catch_backend_error, services::service_product};

#[get("/product")]
pub async fn route_product_get_all(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _viewer_authentication_guard: ViewerAuthenticationGuard
) -> Result<Json<Vec<ProductViewDTO>>, Custom<&'static str>> {

    let result = service_product::get_all_products(database).await;

    match result {
        Ok(products) => Ok(Json(products)),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[get("/product/informations")]
pub async fn route_product_informations(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _viewer_authentication_guard: ViewerAuthenticationGuard
) -> Result<Json<ProductInformationsViewDTO>, Custom<&'static str>> {

    let result = service_product::get_products_informations(database).await;

    match result {
        Ok(products) => Ok(Json(products)),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}


#[post("/product", data="<product_create_dto>")]
pub async fn route_product_create(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _operator_authentication_guard: OperatorAuthenticationGuard,
    product_create_dto: Json<ProductCreateDTO>
) -> Result<Status, Custom<&'static str>> {

    let result = service_product::create_product(database, product_create_dto.0).await;

    match result {
        Ok(_) => Ok(Status::Created),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[put("/product", data="<product_update_dto>")]
pub async fn route_product_update(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _operator_authentication_guard: OperatorAuthenticationGuard,
    product_update_dto: Json<ProductUpdateDTO>
) -> Result<Status, Custom<&'static str>> {

    let result = service_product::update_product(database, product_update_dto.0).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[put("/product/quantity", data="<product_change_quantity_dto>")]
pub async fn route_product_quantity_update(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _operator_authentication_guard: OperatorAuthenticationGuard,
    product_change_quantity_dto: Json<ProductChangeQuantityDTO>
) -> Result<Status, Custom<&'static str>> {

    let result = service_product::change_quantity(database, product_change_quantity_dto.0).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}

#[delete("/product/<product_id>")]
pub async fn route_product_delete(
    database: &State<DatabaseConnection>,
    _authentication_guard: AuthenticationGuard,
    _operator_authentication_guard: OperatorAuthenticationGuard,
    product_id: u64
) -> Result<Status, Custom<&'static str>> {

    let result = service_product::delete_by_id(database, product_id).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error))
    }

}