use rocket::{http::Status, response::status::Custom};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::{dtos::user_dtos::UserCreateDTO, tb_user::{self, ActiveModel, Model}};

pub async fn create_user(database: &DatabaseConnection, user_dto: UserCreateDTO) {

    let user = ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::set(user_dto.get_username().clone()),
        email: ActiveValue::Set(user_dto.get_email().clone()),
        password: ActiveValue::Set(user_dto.get_password().clone()),
        role_id: ActiveValue::Set(user_dto.get_role_id().clone())
    };

    tb_user::Entity::insert(user).exec(database).await.unwrap();

}

pub async fn find_user_by_email(database: &DatabaseConnection, email: String) -> Result<Custom<Model>, Status> {
    let user = tb_user::Entity::find().filter(tb_user::Column::Email.eq(email)).one(database).await;

    // TODO Matchs
    if let Ok(op_model) = user {
        if let Some(model) = op_model {
            return Ok(Custom(Status::Ok, model));
        }
    }

    Err(Status::NotFound)
}