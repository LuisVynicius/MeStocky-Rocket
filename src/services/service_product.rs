use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::{entities::{dtos::product_dtos::{ProductChangeQuantityDTO, ProductCreateDTO, ProductInformationsGetDTO, ProductInformationsViewDTO, ProductUpdateDTO, ProductViewDTO}, tb_product::{self, ActiveModel, Model}}, services::service_report};

pub async fn get_all_products(
    database: &DatabaseConnection,
) -> Vec<ProductViewDTO> {

    let stmt = Statement::from_string(
        DbBackend::MySql, 
    r#"
            SELECT
                tb_product.id,
                tb_product.name,
                tb_product.quantity,
                tb_product.min_quantity,
                tb_category.name as category
            FROM tb_product
            JOIN tb_category
                ON tb_category.id = tb_product.category_id
        "#
    );

    let result = ProductViewDTO::find_by_statement(stmt).all(database).await;

    result.unwrap()

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

pub async fn create_product(
    database: &DatabaseConnection,
    product_create_dto: ProductCreateDTO
) -> Result<&'static str, ()> {

    let product = ActiveModel {
        name: ActiveValue::Set(product_create_dto.get_name().clone()),
        min_quantity: ActiveValue::Set(*product_create_dto.get_min_quantity()),
        category_id: ActiveValue::Set(*product_create_dto.get_category_id()),
        quantity: ActiveValue::Set(0),
        ..Default::default()
    };

    println!("{product:?}");

    let result = tb_product::Entity::insert(product).exec(database).await;

    match result {

        Ok(_) => Ok("Produto criado com sucesso"),
        Err(_) => Err(())

    }

}

pub async fn update_product(
    database: &DatabaseConnection,
    product_update_dto: ProductUpdateDTO
) -> Result<&'static str, ()> {

    if !exists_product_by_id(database, *product_update_dto.get_id()).await {
        return Err(());
    }

    match find_product_by_name(database, product_update_dto.get_name()).await {
        Some(old_product) => {
            if &old_product.id != product_update_dto.get_id() {
                return Err(())
            }
        },
        None => {}
    }

    let product = ActiveModel {
        id: ActiveValue::Set(*product_update_dto.get_id()),
        name: ActiveValue::Set(product_update_dto.get_name().clone()),
        min_quantity: ActiveValue::Set(*product_update_dto.get_min_quantity()),
        category_id: ActiveValue::Set(*product_update_dto.get_category_id()),
        ..Default::default()
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

pub async fn find_product_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Option<Model> {

    let product = tb_product::Entity::find_by_id(id)
        .one(database)
        .await;

    product.unwrap_or(None)

}

async fn find_product_by_name(
    database: &DatabaseConnection,
    name: &str
) -> Option<Model> {

    let product = tb_product::Entity::find()
        .filter(tb_product::Column::Name.eq(name))
        .one(database)
        .await;

    product.unwrap_or(None)

}

async fn exists_product_by_id(
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