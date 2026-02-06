use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::{entities::{dtos::{generic_dtos::ExistsDTO, product_dtos::{ProductChangeQuantityDTO, ProductCreateDTO, ProductInformationsGetDTO, ProductInformationsViewDTO, ProductUpdateDTO, ProductViewDTO}}, tb_product::{self, ActiveModel, Model}}, errors::BackendError, services::service_report};

pub async fn get_all_products(
    database: &DatabaseConnection,
) -> Result<Vec<ProductViewDTO>, BackendError> {

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

    match result {
        Ok(products) => Ok(products),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

pub async fn get_products_informations(
    database: &DatabaseConnection,
) -> Result<ProductInformationsViewDTO, BackendError> {

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
        .await;

    match result {
        Ok(informations_opt) => {
            match informations_opt {
                Some(informations) => Ok(informations.into()),
                None => Err(BackendError::ResourceNotFoundError)
            }
        },
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }
    
}

pub async fn create_product(
    database: &DatabaseConnection,
    product_create_dto: ProductCreateDTO
) -> Result<(), BackendError> {

    match exists_by_name(database, product_create_dto.get_name()).await {
        Ok(boolean) => {
            if boolean {
                return Err(BackendError::ResourceAlreadyInsertedError);
            }
        },
        Err(db_err) => return Err(BackendError::DatabaseError(db_err))
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
        Ok(_) => Ok(()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

pub async fn update_product(
    database: &DatabaseConnection,
    product_update_dto: ProductUpdateDTO
) -> Result<(), BackendError> {

    match exists_by_id(database, product_update_dto.get_id()).await {
        Ok(boolean) => {
            if !boolean {
                return Err(BackendError::ResourceNotFoundError)
            }
        },
        Err(db_err) => return Err(BackendError::DatabaseError(db_err))
    }

    if let Ok(old_product) = find_by_name(database, product_update_dto.get_name()).await {

        if &old_product.id != product_update_dto.get_id() {
            return Err(BackendError::ResourceConflitUpdateError);
        }

    }

    let product = create_update_active_model(product_update_dto);

    let result = tb_product::Entity::update(product).exec(database).await;

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

    let result = tb_product::Entity::delete_by_id(id).exec(database).await;

    match result {
        Ok(_) => Ok(()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

pub async fn change_quantity(
    database: &DatabaseConnection,
    product_change_quantity_dto: ProductChangeQuantityDTO
) -> Result<(), BackendError> {

    match exists_by_id(database, product_change_quantity_dto.get_id()).await {
        Ok(boolean) => {
            if !boolean {
                return Err(BackendError::ResourceNotFoundError)
            }
        },
        Err(db_err) => return Err(BackendError::DatabaseError(db_err))
    }

    let product = match find_product_by_id(database, *product_change_quantity_dto.get_id()).await {
        Ok(model) => model,
        Err(backend_error) => return Err(backend_error)
    };

    let updated_product = create_update_change_active_model(&product_change_quantity_dto, product);

    match updated_product {
        Ok(new_product) => {
            match tb_product::Entity::update(new_product).exec(database).await {
                Ok(_) => {
                    service_report::create_report(database, product_change_quantity_dto).await.unwrap();
                    Ok(())
                },
                Err(db_err) => Err(BackendError::DatabaseError(db_err))
            }
        },
        Err(backend_error) => Err(backend_error)
    }

}

async fn find_product_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Result<Model, BackendError> {

    let result = tb_product::Entity::find_by_id(id)
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

async fn find_by_name(
    database: &DatabaseConnection,
    name: &str
) -> Result<Model, BackendError> {

    let result = tb_product::Entity::find()
        .filter(tb_product::Column::Name.eq(name))
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
                    FROM tb_product
                    WHERE tb_product.id = (\"{id}\")
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
                    FROM tb_product
                    WHERE tb_product.name = (\"{name}\")
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

fn create_update_change_active_model(product_change_quantity_dto: &ProductChangeQuantityDTO, product: Model) -> Result<ActiveModel, BackendError> {

    let active_model = ActiveModel {
        id: ActiveValue::Set(product_change_quantity_dto.get_id().clone()),
        quantity: match product_change_quantity_dto.get_change_type() {
            &true => ActiveValue::Set(product.quantity + product_change_quantity_dto.get_quantity()),
            &false => {
                if product.quantity < *product_change_quantity_dto.get_quantity() {
                    return Err(BackendError::ResourceConflitUpdateError);
                }
                
                ActiveValue::Set(product.quantity - product_change_quantity_dto.get_quantity())
            },
        },
        ..Default::default()
    };

    Ok(active_model)
}