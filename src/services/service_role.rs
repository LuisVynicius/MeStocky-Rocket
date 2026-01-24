use sea_orm::{ActiveModelBehavior, DatabaseConnection, EntityTrait};

use crate::entities::{dtos::role_dtos::RoleCreateViewDTO, tb_role::{self, ActiveModel}};

pub async fn get_all_role(
    database: &DatabaseConnection,
) -> Vec<RoleCreateViewDTO> {

    let roles = tb_role::Entity::find().all(database).await;

    roles.unwrap()
        .into_iter().map(
            |model| RoleCreateViewDTO::new(model.name)
        ).collect()

}

pub async fn create_role(
    database: &DatabaseConnection,
    role_create_dto: RoleCreateViewDTO
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