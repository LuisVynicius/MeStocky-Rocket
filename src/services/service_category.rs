use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::{entities::{dtos::{category_dtos::{CategoryCreateDTO, CategoryDTO, CategoryViewDTO}, generic_dtos::ExistsDTO}, tb_category::{self, ActiveModel, Model}}};

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

    if exists_by_name(database, category_create_dto.get_name()).await {

        return Err(());

    }

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

    if !exists_by_id(database, category_update_dto.get_id()).await {
        return Err(());
    }

    if let Some(old_category) = find_by_name(database, category_update_dto.get_name()).await {
        if &old_category.id != category_update_dto.get_id() {
            return Err(())
        }
    }

    let category = create_update_active_model(category_update_dto);

    let result = tb_category::Entity::update(category).exec(database).await;

    match result {
        Ok(_) => {
            Ok("Categoria atualizada com sucesso")
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

    let result = tb_category::Entity::delete_by_id(id).exec(database).await;

    match result {
        Ok(_) => {
            Ok("Categoria deletada com sucesso")
        },
        Err(_) => Err(())
    }

}

async fn find_by_name(
    database: &DatabaseConnection,
    name: &str
) -> Option<Model> {

    let model = tb_category::Entity::find()
        .filter(tb_category::Column::Name.eq(name))
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
                    FROM tb_category
                    WHERE tb_category.id = (\"{id}\")
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
                    FROM tb_category
                    WHERE tb_category.name = (\"{name}\")
                ) AS 'exist'
        ")
    );

    let result = ExistsDTO::find_by_statement(stmt).one(database).await;
    
    result.unwrap().unwrap().get_into_exist()

}

fn create_update_active_model(category_update_dto: CategoryDTO) -> ActiveModel {
     
     let active_model = ActiveModel {
        name: match category_update_dto.get_name().trim().is_empty() {
            true => ActiveValue::NotSet,
            false => ActiveValue::Set(category_update_dto.get_name().clone())
        },
        ..Default::default()
     };

     active_model

}