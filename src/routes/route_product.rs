use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::product_dtos::ProductCreateDTO, guards::guard_user::Authentication, services::service_product};

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