use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::{entities::{dtos::{generic_dtos::ExistsDTO, product_dtos::{ProductChangeQuantityDTO, ProductCreateDTO, ProductInformationsGetDTO, ProductInformationsViewDTO, ProductUpdateDTO, ProductViewDTO}}, tb_product::{self, ActiveModel, Model}}, services::service_report};

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

    if exists_by_name(database, product_create_dto.get_name()).await {

        return Err(());

    }

    let product = ActiveModel {
        name: ActiveValue::Set(product_create_dto.get_name().clone()),
        min_quantity: ActiveValue::Set(*product_create_dto.get_min_quantity()),
        category_id: ActiveValue::Set(*product_create_dto.get_category_id()),
        quantity: ActiveValue::Set(0),
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
    product_update_dto: ProductUpdateDTO
) -> Result<&'static str, ()> {

    if !exists_by_id(database, product_update_dto.get_id()).await {
        return Err(());
    }

    if let Some(old_product) = find_by_name(database, product_update_dto.get_name()).await {
        if &old_product.id != product_update_dto.get_id() {
            return Err(())
        }
    }

    let product = create_update_active_model(product_update_dto);

    let result = tb_product::Entity::update(product).exec(database).await;

    match result {

        Ok(_) => {
            Ok("Produto atualizado com sucesso")
        },
        Err(_) => Err(())

    }

}

pub async fn delete_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Result<&'static str, ()> {

    if !exists_by_id(database, &id).await {
        return Err(());
    }

    let result = tb_product::Entity::delete_by_id(id).exec(database).await;

    match result {
        Ok(_) => {
            Ok("Produto deletada com sucesso")
        },
        Err(_) => Err(())
    }

}

pub async fn change_quantity(
    database: &DatabaseConnection,
    product_change_quantity_dto: ProductChangeQuantityDTO
) -> Result<&'static str, ()> {

    if !exists_by_id(database, product_change_quantity_dto.get_id()).await {

        return Err(());

    }

    let product = find_product_by_id(database, *product_change_quantity_dto.get_id()).await.unwrap();

    let updated_product = create_update_change_active_model(&product_change_quantity_dto, product);

    match updated_product {
        Ok(new_product) => {
            match tb_product::Entity::update(new_product).exec(database).await {
                Ok(_) => {
                    service_report::create_report(database, product_change_quantity_dto).await.unwrap();
                    Ok("Quantidade alterada com sucesso")
                },
                Err(_) => Err(())
            }
        },
        Err(_) => Err(())
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

async fn find_by_name(
    database: &DatabaseConnection,
    name: &str
) -> Option<Model> {

    let model = tb_product::Entity::find()
        .filter(tb_product::Column::Name.eq(name))
        .one(database)
        .await;

    model.unwrap_or(None)

}

async fn exists_by_id(
    database: &DatabaseConnection,
    id: &u64
) -> bool {
    
    let stmt = Statement::from_string(
        DbBackend::MySql,
        format!("
            SELECT
                EXISTS(
                    SELECT 1
                    FROM tb_product
                    WHERE tb_product.id = (\"{id}\")
                ) AS 'exist'
        ")
    );

    let result = ExistsDTO::find_by_statement(stmt).one(database).await;
    
    result.unwrap().unwrap().get_into_exist()

}

async fn exists_by_name(
    database: &DatabaseConnection,
    name: &str
) -> bool {

    let stmt = Statement::from_string(
        DbBackend::MySql,
        format!("
            SELECT
                EXISTS(
                    SELECT 1
                    FROM tb_product
                    WHERE tb_product.name = (\"{name}\")
                ) AS 'exist'
        ")
    );

    let result = ExistsDTO::find_by_statement(stmt).one(database).await;
    
    result.unwrap().unwrap().get_into_exist()

}

fn create_update_active_model(product_update_dto: ProductUpdateDTO) -> ActiveModel {
     
     let active_model = ActiveModel {
        id: ActiveValue::Set(product_update_dto.get_id().clone()),
        name: match product_update_dto.get_name().trim().is_empty() {
            true => ActiveValue::NotSet,
            false => ActiveValue::Set(product_update_dto.get_name().clone())
        },
        category_id: match product_update_dto.get_category_id() {
            &0 => ActiveValue::NotSet,
            _ => ActiveValue::Set(*product_update_dto.get_category_id())
        },
        min_quantity: match product_update_dto.get_min_quantity() {
            &0 => ActiveValue::NotSet,
            _ => ActiveValue::Set(*product_update_dto.get_min_quantity())
        },
        ..Default::default()
     };

     active_model

}

fn create_update_change_active_model(product_change_quantity_dto: &ProductChangeQuantityDTO, product: Model) -> Result<ActiveModel, ()> {

    let active_model = ActiveModel {
        id: ActiveValue::Set(product_change_quantity_dto.get_id().clone()),
        quantity: match product_change_quantity_dto.get_change_type() {
            &true => ActiveValue::Set(product.quantity + product_change_quantity_dto.get_quantity()),
            &false => {
                if product.quantity < *product_change_quantity_dto.get_quantity() {
                    return Err(());
                }
                
                ActiveValue::Set(product.quantity - product_change_quantity_dto.get_quantity())
            },
        },
        ..Default::default()
    };

    Ok(active_model)
}