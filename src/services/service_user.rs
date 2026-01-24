use sea_orm::{ActiveModelBehavior, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect};

use crate::{configs::config_jwt::{get_email_by_token, valid_token}, entities::{dtos::user_dtos::{UserCreateDTO, UserRoleUpdateDTO, UserUpdateDTO}, tb_user::{self, ActiveModel, Model}}, guards::guard_user::Authentication, services::service_role::exists_role_by_id};

pub async fn create_user(
    database: &DatabaseConnection,
    user_dto: UserCreateDTO
) -> Result<&'static str, ()> {

    if let Some(_) = find_user_by_email(database, user_dto.get_email().clone()).await {
        return Err(());
    }

    let user = ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::set(user_dto.get_username().clone()),
        email: ActiveValue::Set(user_dto.get_email().clone()),
        password: ActiveValue::Set(user_dto.get_password().clone()),
        role_id: ActiveValue::Set(user_dto.get_role_id().clone())
    };

    let result = tb_user::Entity::insert(user)
        .exec(database)
            .await;
    
    match result {
        Ok(_) => Ok("Usuário criado com sucesso"),
        Err(_) => Err(())
    }

}

pub async fn update_user(
    database: &DatabaseConnection,
    user_update_dto: UserUpdateDTO,
    authentication: Authentication
) -> Result<&'static str, ()> {

    let valided_token = valid_token(authentication.0.clone());

    if !valided_token {
        return Err(());
    }

    let email = get_email_by_token(authentication.0);

    let logged_user = find_user_by_email(database, email).await;

    match logged_user {
        Some(model) => {

            let update_user = ActiveModel {
                id: ActiveValue::Set(model.id),
                email: match user_update_dto.get_email() {
                    Some(email) => ActiveValue::Set(email.clone()),
                    None => ActiveValue::default()
                },
                username: match user_update_dto.get_username() {
                    Some(username) => ActiveValue::Set(username.clone()),
                    None => ActiveValue::default()
                },
                ..Default::default()
            };

            match tb_user::Entity::update(update_user).exec(database).await {
                Ok(_) =>return Ok("Usuário atualizado com sucesso"),
                _ => return Err(())
            }

        },
        None => return Err(())
    }

}

pub async fn switch_role(
    database: &DatabaseConnection,
    user_role_update_dto: UserRoleUpdateDTO,
    authentication: Authentication
) -> Result<&'static str, ()> {

    if 
        !exists_user_by_id(database, *user_role_update_dto.get_user_id()).await ||
        !exists_role_by_id(database, *user_role_update_dto.get_role_id()).await

    {
        return Err(());
    }

    let user = ActiveModel {
        id: ActiveValue::Set(*user_role_update_dto.get_user_id()),
        role_id: ActiveValue::Set(*user_role_update_dto.get_role_id()),
        ..Default::default()
    };

    let result = tb_user::Entity::update(user).exec(database).await;

    match result {

        Ok(_) => Ok("Cargo do usuário atualizado com sucesso"),
        Err(_) => Err(())

    }

}

pub async fn find_user_by_email(
    database: &DatabaseConnection,
    email: String
) -> Option<Model> {
    
    let user = tb_user::Entity::find()
        .filter(tb_user::Column::Email.eq(email))
        .one(database).await;

    user.unwrap_or(None)

}

pub async fn find_user_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Option<Model> {

    let user = tb_user::Entity::find_by_id(id)
        .one(database)
        .await;

    user.unwrap_or(None)

}

pub async fn exists_user_by_id(
    database: &DatabaseConnection,
    id: u64
) -> bool {
    
    let result = tb_user::Entity::find_by_id(id)
        .one(database)
        .await;

    match result {
        Ok(model) => model.is_some(),
        Err(_) => false
    }

}