use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, DbErr, EntityTrait, FromQueryResult,
    QueryFilter, Statement,
};

use crate::{
    configs::{
        config_bcrypt::{self, encrypt_password, verify_password},
        config_jwt::{self, generate_token, get_email_by_token},
    },
    entities::{
        dtos::{
            generic_dtos::ExistsDTO,
            user_dtos::{
                AuthenticationDTO, LoginDTO, UserCreateDTO, UserCredentialsUpdateDTO,
                UserInformationsUpdateDTO, UserSummaryForAdminDTO, UserSummaryForAdminQueryDTO,
                ValidedTokenDTO,
            },
        },
        enums::user_enums::UserRole,
        tb_user::{self, ActiveModel, Model},
    },
    errors::BackendError,
    guards::guard_user::AuthenticationGuard,
};

pub async fn login(
    database: &DatabaseConnection,
    login_dto: LoginDTO,
) -> Result<AuthenticationDTO, BackendError> {
    let result = find_by_email(database, login_dto.get_email()).await;

    match result {
        Ok(user) => {
            if !verify_password(login_dto.get_password(), &user.password) {
                return Err(BackendError::InvalidCredentialsError);
            }

            let token = generate_token(user.email.clone());

            Ok(AuthenticationDTO::new(
                token.unwrap(),
                user.role,
                user.username,
                UserRole::code_to_string(user.role),
            ))
        }
        Err(backend_error) => Err(backend_error),
    }
}

pub async fn valid(
    database: &DatabaseConnection,
    authentication: AuthenticationGuard,
) -> Result<ValidedTokenDTO, BackendError> {
    let token_is_valid = config_jwt::valid_token(&authentication.0);
    let result =
        exists_by_email(database, &config_jwt::get_email_by_token(&authentication.0)).await;

    match result {
        Ok(user_exists) => match user_exists {
            true => Ok(ValidedTokenDTO::new(token_is_valid && user_exists)),
            false => Err(BackendError::InvalidCredentialsError),
        },
        Err(db_err) => Err(BackendError::DatabaseError(db_err)),
    }
}

pub async fn get_all_users(
    database: &DatabaseConnection,
) -> Result<Vec<UserSummaryForAdminDTO>, BackendError> {
    let stmt = Statement::from_string(
        DbBackend::MySql,
        r#"
            SELECT 
                tb_user.id,
                tb_user.username,
                tb_user.email,
                tb_user.role
            FROM tb_user
        "#,
    );

    let result = UserSummaryForAdminQueryDTO::find_by_statement(stmt)
        .all(database)
        .await;

    match result {
        Ok(users) => Ok(users.into_iter().map(|user| user.into()).collect()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err)),
    }
}

pub async fn create_user(
    database: &DatabaseConnection,
    user_create_dto: UserCreateDTO,
) -> Result<(), BackendError> {
    if user_create_dto.get_role() == &1
        || user_create_dto.get_role() < &1
        || user_create_dto.get_role() > &4
    {
        return Err(BackendError::InvalidValuesError);
    }

    match exists_by_email(database, user_create_dto.get_email()).await {
        Ok(boolean) => {
            if boolean {
                return Err(BackendError::ResourceAlreadyInsertedError);
            }
        }
        Err(db_err) => return Err(BackendError::DatabaseError(db_err)),
    }

    let user = ActiveModel {
        id: ActiveValue::NotSet,
        username: ActiveValue::set(user_create_dto.get_username().to_string()),
        email: ActiveValue::Set(user_create_dto.get_email().to_string()),
        password: ActiveValue::Set(encrypt_password(user_create_dto.get_password())),
        role: ActiveValue::Set(*user_create_dto.get_role()),
    };

    let result = tb_user::Entity::insert(user).exec(database).await;

    match result {
        Ok(_) => Ok(()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err)),
    }
}

pub async fn update_user_informations(
    database: &DatabaseConnection,
    user_update_dto: UserInformationsUpdateDTO,
    authentication: AuthenticationGuard,
) -> Result<(), BackendError> {
    let email = get_email_by_token(&authentication.0);

    let result = find_by_email(database, &email).await;

    match result {
        Ok(logged_user) => {
            let update_user = create_update_active_model(user_update_dto, logged_user);

            let result = tb_user::Entity::update(update_user).exec(database).await;

            match result {
                Ok(_) => Ok(()),
                Err(db_err) => Err(BackendError::DatabaseError(db_err)),
            }
        }
        Err(backend_error) => Err(backend_error),
    }
}

pub async fn update_user_credentials(
    database: &DatabaseConnection,
    user_update_dto: UserCredentialsUpdateDTO,
    authentication: AuthenticationGuard,
) -> Result<(), BackendError> {
    let email = get_email_by_token(&authentication.0);

    let result = find_by_email(database, &email).await;

    match result {
        Ok(logged_user) => {
            if !config_bcrypt::verify_password(
                user_update_dto.get_old_password(),
                &logged_user.password,
            ) {
                return Err(BackendError::InvalidCredentialsError);
            }

            let update_user = ActiveModel {
                id: ActiveValue::Set(logged_user.id),
                password: ActiveValue::Set(config_bcrypt::encrypt_password(
                    user_update_dto.get_new_password(),
                )),
                ..Default::default()
            };

            let result = tb_user::Entity::update(update_user).exec(database).await;

            match result {
                Ok(_) => Ok(()),
                Err(db_err) => Err(BackendError::DatabaseError(db_err)),
            }
        }
        Err(backend_error) => Err(backend_error),
    }
}

pub async fn delete_user_by_id(database: &DatabaseConnection, id: u64) -> Result<(), BackendError> {
    if id == 1 {
        return Err(BackendError::InvalidValuesError);
    }

    match exists_by_id(database, &id).await {
        Ok(boolean) => {
            if !boolean {
                return Err(BackendError::ResourceNotFoundError);
            }
        }
        Err(db_err) => return Err(BackendError::DatabaseError(db_err)),
    }

    let result = tb_user::Entity::delete_by_id(id).exec(database).await;

    match result {
        Ok(_) => Ok(()),
        Err(db_err) => Err(BackendError::DatabaseError(db_err)),
    }
}

pub async fn find_by_email(
    database: &DatabaseConnection,
    email: &str,
) -> Result<Model, BackendError> {
    let result = tb_user::Entity::find()
        .filter(tb_user::Column::Email.eq(email))
        .one(database)
        .await;

    match result {
        Ok(model_opt) => match model_opt {
            Some(model) => Ok(model),
            None => Err(BackendError::ResourceNotFoundError),
        },
        Err(db_err) => Err(BackendError::DatabaseError(db_err)),
    }
}

async fn exists_by_email(database: &DatabaseConnection, email: &str) -> Result<bool, DbErr> {
    let stmt = Statement::from_string(
        DbBackend::MySql,
        format!(
            "
            SELECT
                EXISTS(
                    SELECT 1
                    FROM tb_user
                    WHERE tb_user.email = (\"{email}\")
                ) AS 'exist'
        "
        ),
    );

    let result = ExistsDTO::find_by_statement(stmt).one(database).await;

    match result {
        Ok(exists_opt) => match exists_opt {
            Some(exists_dto) => Ok(exists_dto.get_into_exist()),
            None => Err(DbErr::RecordNotInserted),
        },
        Err(db_err) => Err(db_err),
    }
}

pub async fn exists_by_id(database: &DatabaseConnection, id: &u64) -> Result<bool, DbErr> {
    let stmt = Statement::from_string(
        DbBackend::MySql,
        format!(
            "
            SELECT
                EXISTS(
                    SELECT 1
                    FROM tb_user
                    WHERE tb_user.id = (\"{id}\")
                ) AS 'exist'
        "
        ),
    );

    let result = ExistsDTO::find_by_statement(stmt).one(database).await;

    match result {
        Ok(exists_opt) => match exists_opt {
            Some(exists_dto) => Ok(exists_dto.get_into_exist()),
            None => Err(DbErr::RecordNotInserted),
        },
        Err(db_err) => Err(db_err),
    }
}

fn create_update_active_model(
    user_update_dto: UserInformationsUpdateDTO,
    logged_user: Model,
) -> ActiveModel {
    let active_model = ActiveModel {
        id: ActiveValue::Set(logged_user.id),
        email: match user_update_dto.get_email().trim().is_empty() {
            true => ActiveValue::NotSet,
            false => ActiveValue::Set(user_update_dto.get_email().to_string()),
        },
        username: match user_update_dto.get_username().trim().is_empty() {
            true => ActiveValue::NotSet,
            false => ActiveValue::Set(user_update_dto.get_username().to_string()),
        },
        ..Default::default()
    };

    active_model
}
