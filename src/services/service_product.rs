use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::{dtos::product_dtos::{ProductCreateDTO, ProductDTO}, tb_product::{self, ActiveModel}};

pub async fn get_all_products(
    database: &DatabaseConnection,
) -> Vec<ProductDTO> {

    let products = tb_product::Entity::find().all(database).await;

    products.unwrap()
        .into_iter().map(
            |model| ProductDTO::new(model.id, model.name, model.quantity, model.min_quantity, model.category_id)
        ).collect()

}

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

pub async fn update_product(
    database: &DatabaseConnection,
    product_update_dto: ProductDTO
) -> Result<&'static str, ()> {

    if !exists_product_by_id(database, *product_update_dto.get_id()).await {
        return Err(());
    }

    if exists_product_by_name(database, product_update_dto.get_name().clone()).await {
        return Err(());
    }

    let product = ActiveModel {
        id: ActiveValue::Set(*product_update_dto.get_id()),
        name: ActiveValue::Set(product_update_dto.get_name().clone()),
        quantity: ActiveValue::Set(*product_update_dto.get_quantity()),
        min_quantity: ActiveValue::Set(*product_update_dto.get_min_quantity()),
        category_id: ActiveValue::Set(*product_update_dto.get_category_id())
    };

    let result = tb_product::Entity::update(product).exec(database).await;

    match result {

        Ok(_) => {
            Ok("Produto atualizado com sucesso")
        },
        Err(_) => Err(())

    }

}

pub async fn exists_product_by_id(
    database: &DatabaseConnection,
    id: u64
) -> bool {
    
    let result = tb_product::Entity::find_by_id(id)
        .one(database)
        .await;

    match result {
        Ok(model) => model.is_some(),
        Err(_) => false
    }

}

pub async fn exists_product_by_name(
    database: &DatabaseConnection,
    name: String
) -> bool {
    
    let result = tb_product::Entity::find()
        .filter(tb_product::Column::Name.eq(name))
        .one(database)
        .await;

    match result {
        Ok(model) => model.is_some(),
        Err(_) => false
    }

}