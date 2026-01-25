use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::{dtos::role_dtos::{RoleCreateDTO, RoleDTO}, tb_role::{self, ActiveModel, Model}};

pub async fn get_all_roles(
    database: &DatabaseConnection,
) -> Vec<RoleDTO> {

    let roles = tb_role::Entity::find().all(database).await;

    roles.unwrap()
        .into_iter().map(
            |model| RoleDTO::new(model.id, model.name)
        ).collect()

}

pub async fn create_role(
    database: &DatabaseConnection,
    role_create_dto: RoleCreateDTO
) -> Result<&'static str, ()> {

    let role = ActiveModel {
        name: sea_orm::ActiveValue::Set(role_create_dto.get_name().clone()),
        ..Default::default()
    };

    let result = tb_role::Entity::insert(role).exec(database).await;

    match result {
        Ok(_) => Ok("Cargo criado com sucesso"),
        Err(_) => Err(())
    }
}

pub async fn update_role(
    database: &DatabaseConnection,
    role_update_dto: RoleDTO
) -> Result<&'static str, ()> {

    if !exists_role_by_id(database, *role_update_dto.get_id()).await {
        return Err(());
    }

    if exists_role_by_name(database, role_update_dto.get_name().clone()).await {
        return Err(());
    }

    let role = ActiveModel {
        id: ActiveValue::Set(role_update_dto.get_id().clone()),
        name: ActiveValue::Set(role_update_dto.get_name().clone())
    };

    let result = tb_role::Entity::update(role).exec(database).await;

    match result {

        Ok(_) => {
            Ok("Cargo atualizado com sucesso")
        },
        Err(_) => Err(())

    }

}

pub async fn find_role_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Option<Model> {

    let role = tb_role::Entity::find_by_id(id)
        .one(database)
        .await;

    role.unwrap_or(None)

}

pub async fn exists_role_by_id(
    database: &DatabaseConnection,
    id: u64
) -> bool {
    
    let result = tb_role::Entity::find_by_id(id)
        .one(database)
        .await;

    match result {
        Ok(model) => model.is_some(),
        Err(_) => false
    }

}

pub async fn exists_role_by_name(
    database: &DatabaseConnection,
    name: String
) -> bool {
    
    let result = tb_role::Entity::find()
        .filter(tb_role::Column::Name.eq(name))
        .one(database)
        .await;

    match result {
        Ok(model) => model.is_some(),
        Err(_) => false
    }

}