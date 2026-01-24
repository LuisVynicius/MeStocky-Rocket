use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entities::{dtos::role_dtos::{RoleCreateDTO, RoleViewDTO}, tb_role::{self, ActiveModel, Model}};

pub async fn get_all_role(
    database: &DatabaseConnection,
) -> Vec<RoleViewDTO> {

    let roles = tb_role::Entity::find().all(database).await;

    roles.unwrap()
        .into_iter().map(
            |model| RoleViewDTO::new(model.id, model.name)
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