use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::product_dtos::{ProductCreateDTO, ProductDTO, ProductViewDTO}, guards::guard_user::Authentication, services::service_product};

#[get("/product")]
pub async fn route_product_get_all(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<Vec<ProductViewDTO>> {

    let products = service_product::get_all_products(database).await;

    Json(products)

}

#[post("/product", data="<product_create_dto>")]
pub async fn route_product_create(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    product_create_dto: Json<ProductCreateDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_product::create_product(database, product_create_dto.0).await;

    match result {
        Ok(message) => Ok(Custom(Status::Created, message)),
        Err(_) => Err(Status::Conflict)
    }

}

#[put("/product", data="<product_update_dto>")]
pub async fn route_product_update(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    product_update_dto: Json<ProductDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_product::update_product(database, product_update_dto.0).await;

    match result {

        Ok(message) => Ok(Custom(Status::Ok, message)),
        Err(_) => Err(Status::Conflict)

    }

}