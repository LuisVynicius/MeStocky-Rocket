use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::{entities::{dtos::{category_dtos::{CategoryCreateDTO, CategoryDTO, CategoryViewDTO}, generic_dtos::ExistsDTO}, tb_category::{self, ActiveModel, Model}}, errors::BackendError};

pub async fn get_all_categories(
    database: &DatabaseConnection
) -> Result<Vec<CategoryDTO>, BackendError> {

    let result = tb_category::Entity::find().all(database).await;

    match result {
        Ok(categories) => Ok(
            categories.into_iter()
                .map(
                    |category| CategoryDTO::new(category.id, category.name)
                )
                .collect()
        ),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

pub async fn get_all_categories_admin(
    database: &DatabaseConnection
) -> Result<Vec<CategoryViewDTO>, BackendError> {

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

    let result = CategoryViewDTO::find_by_statement(stmt).all(database).await;

    match result {
        Ok(categories) => Ok(categories),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

pub async fn create_category(
    database: &DatabaseConnection,
    category_create_dto: CategoryCreateDTO
) -> Result<(), BackendError> {

    match exists_by_name(database, category_create_dto.get_name()).await {
        Ok(boolean) => {
            if boolean {
                return Err(BackendError::ResourceAlreadyInsertedError);
            }
        },
        Err(db_err) => return Err(BackendError::DatabaseError(db_err))
    }

    let category = ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(category_create_dto.get_name().clone())
    };

    let result = tb_category::Entity::insert(category).exec(database).await;

    match result {
        Ok(_) => Ok(()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

pub async fn update_category(
    database: &DatabaseConnection,
    category_update_dto: CategoryDTO
) -> Result<(), BackendError> {

    match exists_by_id(database, category_update_dto.get_id()).await {
        Ok(boolean) => {
            if !boolean {
                return Err(BackendError::ResourceNotFoundError)
            }
        },
        Err(db_err) => return Err(BackendError::DatabaseError(db_err))
    }

    if let Ok(old_category) = find_by_name(database, category_update_dto.get_name()).await {

        if &old_category.id != category_update_dto.get_id() {
            return Err(BackendError::ResourceConflitUpdateError);
        }

    }

    let category = create_update_active_model(category_update_dto);

    let result = tb_category::Entity::update(category).exec(database).await;

    match result {
        Ok(_) => Ok(()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

pub async fn delete_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Result<(), BackendError> {

    match exists_by_id(database, &id).await {
        Ok(boolean) => {
            if !boolean {
                return Err(BackendError::ResourceNotFoundError)
            }
        },
        Err(db_err) => return Err(BackendError::DatabaseError(db_err))
    }

    let result = tb_category::Entity::delete_by_id(id).exec(database).await;

    match result {
        Ok(_) => Ok(()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

async fn find_by_name(
    database: &DatabaseConnection,
    name: &str
) -> Result<Model, BackendError> {

    let result = tb_category::Entity::find()
        .filter(tb_category::Column::Name.eq(name))
        .one(database)
        .await;

    match result {
        Ok(model_opt) => {
            match model_opt {
                Some(model) => Ok(model),
                None => Err(BackendError::ResourceNotFoundError)
            }
        },
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

async fn exists_by_id(
    database: &DatabaseConnection,
    id: &u64
) -> Result<bool, DbErr> {
    
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
    
    match result {
        Ok(exists_opt) => {
            match exists_opt {
                Some(exists_dto) => Ok(exists_dto.get_into_exist()),
                None => Err(DbErr::RecordNotInserted)
            }
        },
        Err(db_err) => Err(db_err)
    }

}

async fn exists_by_name(
    database: &DatabaseConnection,
    name: &str
) -> Result<bool, DbErr> {

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
    
    match result {
        Ok(exists_opt) => {
            match exists_opt {
                Some(exists_dto) => Ok(exists_dto.get_into_exist()),
                None => Err(DbErr::RecordNotInserted)
            }
        },
        Err(db_err) => Err(db_err)
    }

}

fn create_update_active_model(category_update_dto: CategoryDTO) -> ActiveModel {
     
     let active_model = ActiveModel {
        id: ActiveValue::Set(*category_update_dto.get_id()),
        name: match category_update_dto.get_name().trim().is_empty() {
            true => ActiveValue::NotSet,
            false => ActiveValue::Set(category_update_dto.get_name().clone())
        },
     };

     active_model

}