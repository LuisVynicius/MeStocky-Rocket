use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{configs::{config_bcrypt::{encrypt_password, verify_password}, config_jwt::{self, generate_token, get_email_by_token, valid_token}}, entities::{dtos::user_dtos::{AuthenticationDTO, LoginDTO, UserCreateDTO, UserInformationsUpdateDTO, UserRoleUpdateDTO, UserSummaryForAdminDTO, ValidedTokenDTO}, enums::user_enums::UserRole, tb_user::{self, ActiveModel, Model}}, guards::guard_user::Authentication};

pub async fn login(
    database: &DatabaseConnection,
    login_dto: LoginDTO
) -> Result<AuthenticationDTO, ()> {
    
    let user = find_user_by_email(database, login_dto.get_email().clone()).await;

    match user {
     
        Some(user) => {
            if verify_password(login_dto.get_password(), &user.password) {
                let token = generate_token(user.email.clone());

                return Ok(AuthenticationDTO::new(token.unwrap(), user.role, user.username, UserRole::code_to_string(user.role)));
            }
            Err(())
        },
        None => Err(())

    }

}

pub async fn valid(
    database: &DatabaseConnection,
    authentication: Authentication
) -> ValidedTokenDTO { 

    let token_is_valid = config_jwt::valid_token(&authentication.0);
    let user_exists = find_user_by_email(database, config_jwt::get_email_by_token(authentication.0)).await.is_some();

    ValidedTokenDTO::new(token_is_valid && user_exists)

}

pub async fn get_all_users(
    database: &DatabaseConnection
) -> Vec<UserSummaryForAdminDTO> {

    let users = tb_user::Entity::find().all(database).await;

    let users = {
   
        let mut  vec = Vec::new();

        for model in users.unwrap() {

            let role = UserRole::code_to_string(model.role);

            vec.push(
                UserSummaryForAdminDTO::new(model.id, model.username, model.email, role)
            );

        }

        vec
   
    };

    return users;

}

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
        password: ActiveValue::Set(
            encrypt_password(user_dto.get_password())
        ),
        role: ActiveValue::Set(*user_dto.get_role())
    };

    let result = tb_user::Entity::insert(user)
        .exec(database)
        .await;
    
    match result {
        Ok(_) => Ok("Usuário criado com sucesso"),
        Err(_) => Err(())
    }

}

pub async fn update_user_informations(
    database: &DatabaseConnection,
    user_update_dto: UserInformationsUpdateDTO,
    authentication: Authentication
) -> Result<&'static str, ()> {

    let email = get_email_by_token(authentication.0);

    let logged_user = find_user_by_email(database, email).await;

    match logged_user {

        Some(model) => {

            let update_user = ActiveModel {
                id: ActiveValue::Set(model.id),
                email: match user_update_dto.get_email() {
                    Some(email) => {
                        match email.trim().is_empty() {
                            true => ActiveValue::Set(model.email),
                            false => ActiveValue::Set(email.clone())
                        }
                    },
                    None => ActiveValue::Set(model.email)
                },
                username: match user_update_dto.get_username() {
                    Some(username) => {
                        match username.trim().is_empty() {
                            true => ActiveValue::Set(model.username),
                            false => ActiveValue::Set(username.clone())
                        }
                    },
                    None => ActiveValue::Set(model.username)
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

pub async fn delete_user_by_id(
    database: &DatabaseConnection,
    id: u64
) -> Result<&'static str, ()> {

    if !exists_user_by_id(database, id).await {
        return Err(());
    }

    let result = tb_user::Entity::delete_by_id(id).exec(database).await;

    match result {
        Ok(_) => {
            Ok("Usuário deletado com sucesso")
        },
        Err(_) => Err(())
    }

}

pub async fn switch_role(
    database: &DatabaseConnection,
    user_role_update_dto: UserRoleUpdateDTO,
    _authentication: Authentication
) -> Result<&'static str, ()> {

    if !exists_user_by_id(database, *user_role_update_dto.get_user_id()).await {
        return Err(());
    }

    let user = ActiveModel {
        id: ActiveValue::Set(*user_role_update_dto.get_user_id()),
        role: ActiveValue::Set(*user_role_update_dto.get_role()),
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