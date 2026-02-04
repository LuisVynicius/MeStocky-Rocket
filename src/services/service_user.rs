use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, QueryFilter, Statement};

use crate::{configs::{config_bcrypt::{self, encrypt_password, verify_password}, config_jwt::{self, generate_token, get_email_by_token}}, entities::{dtos::{generic_dtos::ExistsDTO, user_dtos::{AuthenticationDTO, LoginDTO, UserCreateDTO, UserCredentialsUpdateDTO, UserInformationsUpdateDTO, UserRoleUpdateDTO, UserSummaryForAdminDTO, UserSummaryForAdminQueryDTO, ValidedTokenDTO}}, enums::user_enums::UserRole, tb_user::{self, ActiveModel, Model}}, guards::guard_user::Authentication};

pub async fn login(
    database: &DatabaseConnection,
    login_dto: LoginDTO
) -> Result<AuthenticationDTO, ()> {
    
    let user = find_by_email(database, login_dto.get_email()).await;

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
    let user_exists = find_by_email(database, &config_jwt::get_email_by_token(authentication.0)).await.is_some();

    ValidedTokenDTO::new(token_is_valid && user_exists)

}

pub async fn get_all_users(
    database: &DatabaseConnection
) -> Vec<UserSummaryForAdminDTO> {

    let stmt = Statement::from_string(
        DbBackend::MySql, 
        r#"
            SELECT 
                tb_user.id,
                tb_user.username,
                tb_user.email,
                tb_user.role
            FROM tb_user
        "#
    );

    let result = UserSummaryForAdminQueryDTO::find_by_statement(stmt).all(database).await;

    result.unwrap()
        .into_iter()
        .map(|user| user.into())
        .collect()

}

pub async fn create_user(
    database: &DatabaseConnection,
    user_create_dto: UserCreateDTO
) -> Result<&'static str, ()> {

    if exists_by_email(database, user_create_dto.get_email()).await {

        return Err(());

    }

    let user = ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::set(user_create_dto.get_username().to_string()),
        email: ActiveValue::Set(user_create_dto.get_email().to_string()),
        password: ActiveValue::Set(
            encrypt_password(user_create_dto.get_password())
        ),
        role: ActiveValue::Set(*user_create_dto.get_role())
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

    let logged_user = find_by_email(database, &email).await;

    match logged_user {
        Some(model) => {
            let update_user = create_update_active_model(user_update_dto, model);

            match tb_user::Entity::update(update_user).exec(database).await {
                Ok(_) =>return Ok("Usuário atualizado com sucesso"),
                _ => return Err(())
            }
        },
        None => return Err(())
    }

}

pub async fn update_user_credentials(
    database: &DatabaseConnection,
    user_update_dto: UserCredentialsUpdateDTO,
    authentication: Authentication
) -> Result<&'static str, ()> {

    let email = get_email_by_token(authentication.0);

    let logged_user = find_by_email(database, &email).await;

    match logged_user {
        Some(model) => {
            println!("Aqui mano");   
            if !config_bcrypt::verify_password(user_update_dto.get_old_password(), &model.password) {
                return Err(());
            }

            let update_user = ActiveModel {
                id: ActiveValue::Set(model.id),
                password: ActiveValue::Set(config_bcrypt::encrypt_password(user_update_dto.get_new_password())),
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

    if !exists_by_id(database, id).await {
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

    if !exists_by_id(database, *user_role_update_dto.get_user_id()).await {
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

async fn find_by_email(
    database: &DatabaseConnection,
    email: &str
) -> Option<Model> {

    let model = tb_user::Entity::find()
        .filter(tb_user::Column::Email.eq(email))
        .one(database)
        .await;

    model.unwrap_or(None)

}

async fn exists_by_email(
    database: &DatabaseConnection,
    email: &str
) -> bool {

    let stmt = Statement::from_string(
        DbBackend::MySql,
        format!("
            SELECT
                EXISTS(
                    SELECT 1
                    FROM tb_user
                    WHERE tb_user.email = (\"{email}\")
                ) AS 'exist'
        ")
    );

    let result = ExistsDTO::find_by_statement(stmt).one(database).await;
    
    result.unwrap().unwrap().get_into_exist()

}

pub async fn exists_by_id(
    database: &DatabaseConnection,
    id: u64
) -> bool {
    
    let stmt = Statement::from_string(
        DbBackend::MySql,
        format!("
            SELECT
                EXISTS(
                    SELECT 1
                    FROM tb_user
                    WHERE tb_user.id = (\"{id}\")
                ) AS 'exist'
        ")
    );

    let result = ExistsDTO::find_by_statement(stmt).one(database).await;
    
    result.unwrap().unwrap().get_into_exist()

}

fn create_update_active_model(user_update_dto: UserInformationsUpdateDTO, logged_user: Model) -> ActiveModel {
     
     let active_model = ActiveModel {
        id: ActiveValue::Set(logged_user.id),
        email: match user_update_dto.get_email().trim().is_empty() {
            true => ActiveValue::NotSet,
            false => ActiveValue::Set(user_update_dto.get_email().to_string())
        },
        username: match user_update_dto.get_username().trim().is_empty() {
            true => ActiveValue::NotSet,
            false => ActiveValue::Set(user_update_dto.get_username().to_string())
        },
        ..Default::default()
     };

     active_model

}