use rocket::{State, http::Status, response::status::{self, Custom}, serde::json::Json};
use sea_orm::DatabaseConnection;

use crate::{entities::dtos::user_dtos::{AuthenticationDTO, LoginDTO, UserCreateDTO, UserCredentialsUpdateDTO, UserInformationsUpdateDTO, UserRoleUpdateDTO, UserSummaryForAdminDTO, ValidedTokenDTO}, guards::guard_user::{AuthenticationGuard, MannagerAuthenticationGuard}, services::service_user::{self}};

#[get("/user")]
pub async fn route_user_get_all(
    database: &State<DatabaseConnection>,
    _authentication: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard

) -> Json<Vec<UserSummaryForAdminDTO>> {

    let users = service_user::get_all_users(database).await;

    Json(users)

}

#[post("/login", data="<login_dto>")]
pub async fn route_login(
    database: &State<DatabaseConnection>,
    login_dto: Json<LoginDTO>
) -> Result<status::Custom<Json<AuthenticationDTO>>, Status> {
    
    let result = service_user::login(database, login_dto.0).await;

    match result {
        Ok(token) => Ok(Custom(Status::Ok, Json(token))),
        Err(_) => Err(Status::Conflict)
    }

}

#[get("/login/valid")]
pub async fn route_valid_token(
    database: &State<DatabaseConnection>,
    authentication: AuthenticationGuard
) -> Result<status::Custom<Json<ValidedTokenDTO>>, Status> {

    let result = service_user::valid(database, authentication).await;

    match result.get_valided() {
        &false => Err(Status::Forbidden),
        &true => Ok(Custom(Status::Ok, Json(ValidedTokenDTO::new(true))))
    }

}

#[post("/user", data="<user_create_dto>")]
pub async fn route_user_create(
    database: &State<DatabaseConnection>,
    user_create_dto: Json<UserCreateDTO>,
    _mannager_authentication_guard: MannagerAuthenticationGuard
) -> Status {

    let result = service_user::create_user(database, user_create_dto.0).await;

    match result { 
        Ok(_) => Status::Created,
        Err(_) => Status::Conflict
    }

}

#[put("/user/informations", data="<user_update_dto>")]
pub async fn route_user_update_informations(
    database: &State<DatabaseConnection>,
    authentication: AuthenticationGuard,
    user_update_dto: Json<UserInformationsUpdateDTO>
) -> Status {

    let result = service_user::update_user_informations(database, user_update_dto.0, authentication).await;

    match result {
        Ok(_) => Status::Ok,
        Err(_) => Status::Forbidden
    }

}

#[put("/user/credentials", data="<user_update_dto>")]
pub async fn route_user_update_credentials(
    database: &State<DatabaseConnection>,
    authentication: AuthenticationGuard,
    user_update_dto: Json<UserCredentialsUpdateDTO>
) -> Status {

    let result = service_user::update_user_credentials(database, user_update_dto.0, authentication).await;

    match result {
        Ok(_) => Status::Ok,
        Err(_) => Status::Forbidden
    }

}

#[put("/user/role", data="<user_role_update_dto>")]
pub async fn route_user_role_update(
    database: &State<DatabaseConnection>,
    authentication: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
    user_role_update_dto: Json<UserRoleUpdateDTO>
) -> Result<Custom<&'static str>, Status> {

    let result = service_user::switch_role(database, user_role_update_dto.0, authentication).await;

    match result {
        Ok(message) => Ok(Custom(Status::Ok, message)),
        Err(_) => Err(Status::Forbidden)
    }

}

#[delete("/user/<user_id>")]
pub async fn route_user_delete(
    database: &State<DatabaseConnection>,
    _authentication: AuthenticationGuard,
    _mannager_authentication_guard: MannagerAuthenticationGuard,
    user_id: u64
) -> Status {

    let result = service_user::delete_user_by_id(database, user_id).await;

    match result {

        Ok(_) => Status::Ok,
        Err(_) => Status::Conflict

    }

}