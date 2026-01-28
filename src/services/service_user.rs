use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{configs::{config_bcrypt::{encrypt_password, verify_password}, config_jwt::{generate_token, get_email_by_token, valid_token}}, entities::{dtos::user_dtos::{AuthenticationDTO, LoginDTO, UserCreateDTO, UserInformationsUpdateDTO, UserRoleUpdateDTO, UserSummaryForAdminDTO}, tb_user::{self, ActiveModel, Model}}, guards::guard_user::Authentication, services::service_role::{self, exists_role_by_id}};

pub async fn login(
    database: &DatabaseConnection,
    login_dto: LoginDTO
) -> Result<AuthenticationDTO, ()> {
    
    let user = find_user_by_email(database, login_dto.get_email().clone()).await;

    match user {
     
        Some(user) => {
            if verify_password(login_dto.get_password(), &user.password) {
                let token = generate_token(user.email.clone());

                return Ok(AuthenticationDTO::new(token.unwrap()));
            }
            Err(())
        },
        None => Err(())

    }

}

pub async fn get_all_users(
    database: &DatabaseConnection
) -> Vec<UserSummaryForAdminDTO> {

    let users = tb_user::Entity::find().all(database).await;

    let users = {
   
        let mut  vec = Vec::new();

        for model in users.unwrap() {

            let role = service_role::find_role_by_id(database, model.role_id).await.unwrap();

            vec.push(
                UserSummaryForAdminDTO::new(model.id, model.username, model.email, role.name)
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

pub async fn update_user_informations(
    database: &DatabaseConnection,
    user_update_dto: UserInformationsUpdateDTO,
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
    _authentication: Authentication
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