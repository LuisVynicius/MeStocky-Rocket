use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::{entities::{dtos::{generic_dtos::ExistsDTO, reason_dtos::{ReasonCreateDTO, ReasonDTO}}, tb_reason::{self, ActiveModel, Model}}, errors::BackendError};

pub async fn get_all_reason(
    database: &DatabaseConnection,
) -> Result<Vec<ReasonDTO>, BackendError> {

    let result = tb_reason::Entity::find().all(database).await;

    match result {
        Ok(reasons) => Ok(
            reasons.into_iter()
                .map(
                    |model| ReasonDTO::new(model.id, model.name)
                )
                .collect()
        ),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

pub async fn create_reason(
    database: &DatabaseConnection,
    reason_create_dto: ReasonCreateDTO
) -> Result<(), BackendError> {

    match exists_by_name(database, reason_create_dto.get_name()).await {
        Ok(boolean) => {
            if boolean {
                return Err(BackendError::ResourceAlreadyInsertedError);
            }
        },
        Err(db_err) => return Err(BackendError::DatabaseError(db_err))
    }

    let reason = ActiveModel {
        name: sea_orm::ActiveValue::Set(reason_create_dto.get_name().clone()),
        ..Default::default()
    };

    let result = tb_reason::Entity::insert(reason).exec(database).await;

    match result {
        Ok(_) => Ok(()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }
}

pub async fn update_reason(
    database: &DatabaseConnection,
    reason_update_dto: ReasonDTO
) -> Result<(), BackendError> {

    match exists_by_id(database, reason_update_dto.get_id()).await {
        Ok(boolean) => {
            if !boolean {
                return Err(BackendError::ResourceNotFoundError)
            }
        },
        Err(db_err) => return Err(BackendError::DatabaseError(db_err))
    }

    if let Ok(old_reason) = find_by_name(database, reason_update_dto.get_name()).await {

        if &old_reason.id != reason_update_dto.get_id() {
            return Err(BackendError::ResourceConflitUpdateError);
        }

    }

    let reason = create_update_active_model(reason_update_dto);

    let result = tb_reason::Entity::update(reason).exec(database).await;

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

    let result = tb_reason::Entity::delete_by_id(id).exec(database).await;

    match result {
        Ok(_) => Ok(()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err))
    }

}

async fn find_by_name(
    database: &DatabaseConnection,
    name: &str
) -> Result<Model, BackendError> {

    let result = tb_reason::Entity::find()
        .filter(tb_reason::Column::Name.eq(name))
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
                    FROM tb_reason
                    WHERE tb_reason.id = (\"{id}\")
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
                    FROM tb_reason
                    WHERE tb_reason.name = (\"{name}\")
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

fn create_update_active_model(reason_update_dto: ReasonDTO) -> ActiveModel {
     
     let active_model = ActiveModel {
        id: ActiveValue::Set(*reason_update_dto.get_id()),
        name: match reason_update_dto.get_name().trim().is_empty() {
            true => ActiveValue::NotSet,
            false => ActiveValue::Set(reason_update_dto.get_name().clone())
        }
     };

     active_model

}