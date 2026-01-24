use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::{dtos::user_dtos::UserCreateDTO, tb_user::{self, ActiveModel, Model}};

pub async fn create_user(database: &DatabaseConnection, user_dto: UserCreateDTO) -> Result<&'static str, ()> {

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

pub async fn find_user_by_email(database: &DatabaseConnection, email: String) -> Option<Model> {
    
    let user = tb_user::Entity::find()
        .filter(tb_user::Column::Email.eq(email))
        .one(database).await;

    user.unwrap_or(None)

}