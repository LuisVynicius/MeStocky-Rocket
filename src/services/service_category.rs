use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::entities::{dtos::category_dtos::{CategoryCreateDTO, CategoryDTO, CategoryViewDTO}, tb_category::{self, ActiveModel, Model}};

pub async fn get_all_categories(
    database: &DatabaseConnection
) -> Vec<CategoryDTO> {

    let categories = tb_category::Entity::find().all(database).await;

    categories.unwrap()
        .into_iter()
        .map(|model| CategoryDTO::new(model.id, model.name) )
        .collect()

}

pub async fn get_all_categories_admin(
    database: &DatabaseConnection
) -> Vec<CategoryViewDTO> {

    let stmt = Statement::from_string(
        DbBackend::MySql,
        r#"
            SELECT
                id,
                name,
                CAST(
                    (
                        SELECT COUNT(*) FROM tb_product WHERE tb_product.category_id = tb_category.id 
                    ) AS UNSIGNED
                ) AS quantity
            FROM tb_category;
        "#
    );

    let categories = CategoryViewDTO::find_by_statement(stmt).all(database).await;

    categories.unwrap()

}

pub async fn create_category(
    database: &DatabaseConnection,
    category_create_dto: CategoryCreateDTO
) -> Result<&'static str, ()> {

    let category = ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(category_create_dto.get_name().clone())
    };

    let result = tb_category::Entity::insert(category).exec(database).await;

    match result {

        Ok(_) => Ok("Categoria criada com sucesso"),
        Err(_) => Err(())

    }

}

pub async fn update_category(
    database: &DatabaseConnection,
    category_update_dto: CategoryDTO
) -> Result<&'static str, ()> {

    if !exists_category_by_id(database, *category_update_dto.get_id()).await {
        return Err(());
    }

    if exists_category_by_name(database, category_update_dto.get_name().clone()).await {
        return Err(());
    }

    let category = ActiveModel {
        id: ActiveValue::Set(category_update_dto.get_id().clone()),
        name: ActiveValue::Set(category_update_dto.get_name().clone())
    };

    let result = tb_category::Entity::update(category).exec(database).await;

    match result {
        Ok(_) => {
            Ok("Categoria atualizada com sucesso")
        },
        Err(_) => Err(())
    }

}

pub async fn delete_category_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Result<&'static str, ()> {

    if !exists_category_by_id(database, id).await {
        return Err(());
    }

    let result = tb_category::Entity::delete_by_id(id).exec(database).await;

    match result {
        Ok(_) => {
            Ok("Categoria deletada com sucesso")
        },
        Err(_) => Err(())
    }

}

pub async fn find_category_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Option<Model> {

    let category = tb_category::Entity::find_by_id(id)
        .one(database)
        .await;

    category.unwrap_or(None)

}

pub async fn exists_category_by_id(
    database: &DatabaseConnection,
    id: u64
) -> bool {
    
    let result = tb_category::Entity::find_by_id(id)
        .one(database)
        .await;

    match result {
        Ok(model) => model.is_some(),
        Err(_) => false
    }

}

pub async fn exists_category_by_name(
    database: &DatabaseConnection,
    name: String
) -> bool {
    
    let result = tb_category::Entity::find()
        .filter(tb_category::Column::Name.eq(name))
        .one(database)
        .await;
    
    match result {
        Ok(model) => model.is_some(),
        Err(_) => false
    }

}