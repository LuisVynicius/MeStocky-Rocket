use sea_orm::{ActiveModelBehavior, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{configs::config_jwt::{get_email_by_token, valid_token}, entities::{dtos::user_dtos::{UserCreateDTO, UserUpdateDTO}, tb_user::{self, ActiveModel, Model}}, guards::guard_user::Authentication};

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

            let mut update_user = ActiveModel::new();

            update_user.id = ActiveValue::Set(model.id);
            
            if let Some(username) = user_update_dto.get_username() {
                update_user.username = ActiveValue::Set(username.clone());
            }

            if let Some(email) = user_update_dto.get_email() {
                update_user.email = ActiveValue::Set(email.clone());
            }

            match tb_user::Entity::update(update_user).exec(database).await {
                Ok(_) =>return Ok("Usuário atualizado com sucesso"),
                _ => return Err(())
            }

        },
        None => return Err(())
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