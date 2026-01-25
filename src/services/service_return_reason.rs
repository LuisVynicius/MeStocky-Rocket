use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::{dtos::return_reason_dtos::{ReturnReasonCreateDTO, ReturnReasonDTO}, tb_return_reason::{self, ActiveModel, Model}};

pub async fn get_all_reason(
    database: &DatabaseConnection,
) -> Vec<ReturnReasonDTO> {

    let reasons = tb_return_reason::Entity::find().all(database).await;

    reasons.unwrap()
        .into_iter().map(
            |model| ReturnReasonDTO::new(model.id, model.name)
        ).collect()

}

pub async fn create_reason(
    database: &DatabaseConnection,
    reason_create_dto: ReturnReasonCreateDTO
) -> Result<&'static str, ()> {

    let reason = ActiveModel {
        name: sea_orm::ActiveValue::Set(reason_create_dto.get_name().clone()),
        ..Default::default()
    };

    let result = tb_return_reason::Entity::insert(reason).exec(database).await;

    match result {
        Ok(_) => Ok("Motivo criado com sucesso"),
        Err(_) => Err(())
    }
}

pub async fn update_reason(
    database: &DatabaseConnection,
    reason_update_dto: ReturnReasonDTO
) -> Result<&'static str, ()> {

    if !exists_reason_by_id(database, *reason_update_dto.get_id()).await {
        return Err(());
    }

    if exists_reason_by_name(database, reason_update_dto.get_name().clone()).await {
        return Err(());
    }

    let reason = ActiveModel {
        id: ActiveValue::Set(reason_update_dto.get_id().clone()),
        name: ActiveValue::Set(reason_update_dto.get_name().clone())
    };

    let result = tb_return_reason::Entity::update(reason).exec(database).await;

    match result {

        Ok(_) => {
            Ok("Motivo atualizado com sucesso")
        },
        Err(_) => Err(())

    }

}

pub async fn find_reason_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Option<Model> {

    let reason = tb_return_reason::Entity::find_by_id(id)
        .one(database)
        .await;

    reason.unwrap_or(None)

}

pub async fn exists_reason_by_id(
    database: &DatabaseConnection,
    id: u64
) -> bool {
    
    let result = tb_return_reason::Entity::find_by_id(id)
        .one(database)
        .await;

    match result {
        Ok(model) => model.is_some(),
        Err(_) => false
    }

}

pub async fn exists_reason_by_name(
    database: &DatabaseConnection,
    name: String
) -> bool {
    
    let result = tb_return_reason::Entity::find()
        .filter(tb_return_reason::Column::Name.eq(name))
        .one(database)
        .await;

    match result {
        Ok(model) => model.is_some(),
        Err(_) => false
    }

}