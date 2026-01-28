use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::{entities::{dtos::product_dtos::{ProductChangeQuantityDTO, ProductCreateDTO, ProductDTO, ProductInformationsGetDTO, ProductInformationsViewDTO, ProductViewDTO}, tb_product::{self, ActiveModel, Model}}, services::{service_category, service_report}};

pub async fn get_all_products(
    database: &DatabaseConnection,
) -> Vec<ProductViewDTO> {

    let products = tb_product::Entity::find().all(database).await;

    let products = {
   
        let mut  vec = Vec::new();

        for model in products.unwrap() {

            let category = service_category::find_category_by_id(database, model.category_id).await.unwrap();

            vec.push(
                ProductViewDTO::new(model.id, model.name, model.quantity, model.min_quantity, category.name)
            );

        }

        vec
   
    };

    return products;

}

pub async fn create_product(
    database: &DatabaseConnection,
    product_create_dto: ProductCreateDTO
) -> Result<&'static str, ()> {

    let product = ActiveModel {
        name: ActiveValue::Set(product_create_dto.get_name().clone()),
        min_quantity: ActiveValue::Set(*product_create_dto.get_min_quantity()),
        category_id: ActiveValue::Set(*product_create_dto.get_category_id()),
        ..Default::default()
    };

    let result = tb_product::Entity::insert(product).exec(database).await;

    match result {

        Ok(_) => Ok("Produto criado com sucesso"),
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

pub async fn change_quantity(
    database: &DatabaseConnection,
    product_change_quantity_dto: ProductChangeQuantityDTO
) -> Result<&'static str, ()> {

    if !exists_product_by_id(database, *product_change_quantity_dto.get_id()).await {

        return Err(());

    }

    let product = find_product_by_id(database, *product_change_quantity_dto.get_id()).await;

    match product {

        Some(product) => {
            let mut updated_product = ActiveModel::default();

            updated_product.id = ActiveValue::Set(product.id);

            let result;

            match *product_change_quantity_dto.get_change_type() {

                false => {
                    if product.quantity < *product_change_quantity_dto.get_quantity() {
                        return Err(());
                    }
                    
                    updated_product.quantity = ActiveValue::Set(product.quantity - product_change_quantity_dto.get_quantity());

                    result = tb_product::Entity::update(updated_product).exec(database).await;
                },
                true => {
                    updated_product.quantity = ActiveValue::Set(product.quantity + product_change_quantity_dto.get_quantity());

                    result = tb_product::Entity::update(updated_product).exec(database).await;
                }

            }

            match result {

                Ok(_) => {
                    service_report::create_report(database, product_change_quantity_dto).await.unwrap();
                    Ok("Quantidade alterada com sucesso")
                },
                Err(_) => Err(())

            }
        },
        None => Err(())

    }

}

pub async fn get_products_informations(
    database: &DatabaseConnection,
) -> ProductInformationsViewDTO {

    let stmt = Statement::from_string(
        DbBackend::MySql,
        r#"
        SELECT
            CAST(COUNT(*) AS UNSIGNED) AS quantity,
            CAST(SUM(quantity) AS UNSIGNED) AS total,
            CAST(SUM(quantity < min_quantity) AS UNSIGNED) AS warnings
        FROM tb_product
        "#.to_owned(),
    );

    let result = ProductInformationsGetDTO::find_by_statement(stmt)
        .one(database)
        .await
        .unwrap();

    result.unwrap().into()
    
}

pub async fn find_product_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Option<Model> {

    let product = tb_product::Entity::find_by_id(id)
        .one(database)
        .await;

    product.unwrap_or(None)

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