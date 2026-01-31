use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::product_dtos::{ProductChangeQuantityDTO, ProductCreateDTO, ProductInformationsViewDTO, ProductUpdateDTO, ProductViewDTO}, guards::guard_user::Authentication, services::service_product};

#[get("/product")]
pub async fn route_product_get_all(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<Vec<ProductViewDTO>> {

    let products = service_product::get_all_products(database).await;

    Json(products)

}

#[get("/product/informations")]
pub async fn route_product_informations(
    database: &State<DatabaseConnection>,
    _authentication: Authentication
) -> Json<ProductInformationsViewDTO> {

    let products = service_product::get_products_informations(database).await;

    Json(products)

}


#[post("/product", data="<product_create_dto>")]
pub async fn route_product_create(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    product_create_dto: Json<ProductCreateDTO>
) -> Status {

    let result = service_product::create_product(database, product_create_dto.0).await;

    match result {
        Ok(_) => Status::Created,
        Err(_) => Status::Conflict
    }

}

#[put("/product", data="<product_update_dto>")]
pub async fn route_product_update(
    database: &State<DatabaseConnection>,
    _authentication: Authentication,
    product_update_dto: Json<ProductUpdateDTO>
) -> Status {

    let result = service_product::update_product(database, product_update_dto.0).await;

    match result {

        Ok(_) => Status::Ok,
        Err(_) => Status::Conflict

    }

}

#[put("/product/quantity", data="<product_change_quantity_dto>")]
pub async fn route_product_quantity_update(
    database: &State<DatabaseConnection>,
    product_change_quantity_dto: Json<ProductChangeQuantityDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_product::change_quantity(database, product_change_quantity_dto.0).await;

    match result {

        Ok(message) => Ok(Custom(Status::Ok, message)),
        Err(_) => Err(Status::Conflict)

    }

}