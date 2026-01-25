use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

use crate::entities::{dtos::product_dtos::ProductCreateDTO, tb_product::{self, ActiveModel}};


pub async fn create_product(
    database: &DatabaseConnection,
    product_create_dto: ProductCreateDTO
) -> Result<&'static str, ()> {

    let product = ActiveModel {
        name: ActiveValue::Set(product_create_dto.get_name().clone()),
        quantity: ActiveValue::Set(*product_create_dto.get_quantity()),
        min_quantity: ActiveValue::Set(*product_create_dto.get_min_quantity()),
        category_id: ActiveValue::Set(*product_create_dto.get_category_id()),
        ..Default::default()
    };

    let result = tb_product::Entity::insert(product).exec(database).await;

    match result {

        Ok(_) => Ok("Produto criada com sucesso"),
        Err(_) => Err(())

    }

}