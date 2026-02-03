use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::entities::{dtos::{generic_dtos::ExistsDTO, reason_dtos::{ReasonCreateDTO, ReasonDTO}}, tb_reason::{self, ActiveModel, Model}};

pub async fn get_all_reason(
    database: &DatabaseConnection,
) -> Vec<ReasonDTO> {

    let reasons = tb_reason::Entity::find().all(database).await;

    reasons.unwrap()
        .into_iter().map(
            |model| ReasonDTO::new(model.id, model.name)
        ).collect()

}

pub async fn create_reason(
    database: &DatabaseConnection,
    reason_create_dto: ReasonCreateDTO
) -> Result<&'static str, ()> {

    if exists_by_name(database, reason_create_dto.get_name()).await {

        return Err(());

    }

    let reason = ActiveModel {
        name: sea_orm::ActiveValue::Set(reason_create_dto.get_name().clone()),
        ..Default::default()
    };

    let result = tb_reason::Entity::insert(reason).exec(database).await;

    match result {
        Ok(_) => Ok("Motivo criado com sucesso"),
        Err(_) => Err(())
    }
}

pub async fn update_reason(
    database: &DatabaseConnection,
    reason_update_dto: ReasonDTO
) -> Result<&'static str, ()> {

    if !exists_by_id(database, reason_update_dto.get_id()).await {
        return Err(());
    }

    if let Some(old_reason) = find_by_name(database, reason_update_dto.get_name()).await {
        if &old_reason.id != reason_update_dto.get_id() {
            return Err(())
        }
    }

    let reason = create_update_active_model(reason_update_dto);

    let result = tb_reason::Entity::update(reason).exec(database).await;

    match result {

        Ok(_) => {
            Ok("Motivo atualizado com sucesso")
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

    let result = tb_reason::Entity::delete_by_id(id).exec(database).await;

    match result {
        Ok(_) => {
            Ok("Motivo deletado com sucesso")
        },
        Err(_) => Err(())
    }

}

async fn find_by_name(
    database: &DatabaseConnection,
    name: &str
) -> Option<Model> {

    let model = tb_reason::Entity::find()
        .filter(tb_reason::Column::Name.eq(name))
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
                    FROM tb_reason
                    WHERE tb_reason.id = (\"{id}\")
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
                    FROM tb_reason
                    WHERE tb_reason.name = (\"{name}\")
                ) AS 'exist'
        ")
    );

    let result = ExistsDTO::find_by_statement(stmt).one(database).await;
    
    result.unwrap().unwrap().get_into_exist()

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