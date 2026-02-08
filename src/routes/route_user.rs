use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{
    entities::dtos::user_dtos::{
        AuthenticationDTO, LoginDTO, UserCreateDTO, UserCredentialsUpdateDTO,
        UserInformationsUpdateDTO, UserSummaryForAdminDTO, ValidedTokenDTO,
    },
    guards::guard_user::{AuthenticationGuard, MannagerAuthenticationGuard},
    routes::generic_functions::catch_backend_error,
    services::service_user::{self},
};

#[get("/user")]
pub async fn route_user_get_all(
    database: &State<DatabaseConnection>,
    _authentication: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
) -> Result<Json<Vec<UserSummaryForAdminDTO>>, Custom<&'static str>> {
    let result = service_user::get_all_users(database).await;

    match result {
        Ok(users) => Ok(Json(users)),
        Err(backend_error) => Err(catch_backend_error(backend_error)),
    }
}

#[post("/login", data = "<login_dto>")]
pub async fn route_login(
    database: &State<DatabaseConnection>,
    login_dto: Json<LoginDTO>,
) -> Result<Json<AuthenticationDTO>, Custom<&'static str>> {
    let result = service_user::login(database, login_dto.0).await;

    match result {
        Ok(token) => Ok(Json(token)),
        Err(backend_error) => Err(catch_backend_error(backend_error)),
    }
}

#[get("/login/valid")]
pub async fn route_valid_token(
    database: &State<DatabaseConnection>,
    authentication: AuthenticationGuard,
) -> Result<Json<ValidedTokenDTO>, Custom<&'static str>> {
    let result = service_user::valid(database, authentication).await;

    match result {
        Ok(valided_token_dto) => Ok(Json(valided_token_dto)),
        Err(backend_error) => Err(catch_backend_error(backend_error)),
    }
}

#[post("/user", data = "<user_create_dto>")]
pub async fn route_user_create(
    database: &State<DatabaseConnection>,
    user_create_dto: Json<UserCreateDTO>,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
) -> Result<Status, Custom<&'static str>> {
    let result = service_user::create_user(database, user_create_dto.0).await;

    match result {
        Ok(_) => Ok(Status::Created),
        Err(backend_error) => Err(catch_backend_error(backend_error)),
    }
}

#[put("/user/informations", data = "<user_update_dto>")]
pub async fn route_user_update_informations(
    database: &State<DatabaseConnection>,
    authentication: AuthenticationGuard,
    user_update_dto: Json<UserInformationsUpdateDTO>,
) -> Result<Status, Custom<&'static str>> {
    let result =
        service_user::update_user_informations(database, user_update_dto.0, authentication).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error)),
    }
}

#[put("/user/credentials", data = "<user_update_dto>")]
pub async fn route_user_update_credentials(
    database: &State<DatabaseConnection>,
    authentication: AuthenticationGuard,
    user_update_dto: Json<UserCredentialsUpdateDTO>,
) -> Result<Status, Custom<&'static str>> {
    let result =
        service_user::update_user_credentials(database, user_update_dto.0, authentication).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error)),
    }
}

#[delete("/user/<user_id>")]
pub async fn route_user_delete(
    database: &State<DatabaseConnection>,
    _authentication: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
    user_id: u64,
) -> Result<Status, Custom<&'static str>> {
    let result = service_user::delete_user_by_id(database, user_id).await;

    match result {
        Ok(_) => Ok(Status::Ok),
        Err(backend_error) => Err(catch_backend_error(backend_error)),
    }
}
