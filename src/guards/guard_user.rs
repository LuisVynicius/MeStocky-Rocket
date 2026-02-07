use rocket::{State, http::Status, request::{FromRequest, Outcome, Request}};
use sea_orm::DatabaseConnection;

use crate::{configs::config_jwt::{self, valid_token}, services::service_user};

#[derive(Default)]
pub struct AuthenticationGuard(pub String);

#[async_trait]
impl<'r> FromRequest<'r> for AuthenticationGuard {
    
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        
        let token = req.headers().get_one("token");

        match token {
            Some(token) => match valid_token(token) {
                true => Outcome::Success(AuthenticationGuard(token.to_string())),
                false => Outcome::Error((Status::Forbidden, "Token de autenticação inválido"))
            },
            None => Outcome::Error((Status::Forbidden, "Token de autenticação não encontrado, tente realizar o login"))
        }

    }

}

#[derive(Default)]
pub struct AdminAuthenticationGuard;

#[async_trait]
impl<'r> FromRequest<'r> for AdminAuthenticationGuard {

    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let token = req.headers().get_one("token").unwrap();

        let database = req.guard::<&State<DatabaseConnection>>().await;
        
        let email = config_jwt::get_email_by_token(token);

        match database {
            Outcome::Success(database) => {
                verify_role(database.inner(), &email, 1).await
            },
            _ => Outcome::Error((Status::BadRequest, "Erro no banco de dados"))
        }

    }
}

#[derive(Default)]
pub struct MannagerAuthenticationGuard;

#[async_trait]
impl<'r> FromRequest<'r> for MannagerAuthenticationGuard {

    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let token = req.headers().get_one("token").unwrap();

        let database = req.guard::<&State<DatabaseConnection>>().await;
        
        let email = config_jwt::get_email_by_token(token);

        match database {
            Outcome::Success(database) => {
                verify_role(database.inner(), &email, 2).await
            },
            _ => Outcome::Error((Status::InternalServerError, "Erro no banco de dados"))
        }

    }
}

#[derive(Default)]
pub struct OperatorAuthenticationGuard;

#[async_trait]
impl<'r> FromRequest<'r> for OperatorAuthenticationGuard {

    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let token = req.headers().get_one("token").unwrap();

        let database = req.guard::<&State<DatabaseConnection>>().await;
        
        let email = config_jwt::get_email_by_token(token);

        match database {
            Outcome::Success(database) => {
                verify_role(database.inner(), &email, 3).await
            },
            _ => Outcome::Error((Status::InternalServerError, "Erro no banco de dados"))
        }

    }
}

#[derive(Default)]
pub struct ViewerAuthenticationGuard;

#[async_trait]
impl<'r> FromRequest<'r> for ViewerAuthenticationGuard {

    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        let token = req.headers().get_one("token").unwrap();

        let database = req.guard::<&State<DatabaseConnection>>().await;
        
        let email = config_jwt::get_email_by_token(token);

        match database {
            Outcome::Success(database) => {
                verify_role(database.inner(), &email, 4).await
            },
            _ => Outcome::Error((Status::InternalServerError, "Erro no banco de dados"))
        }

    }
}

async fn verify_role<T: Default>(database: &DatabaseConnection, email: &str, role: u8) -> Outcome<T, &'static str> {
    
    let result = service_user::find_by_email(database, &email).await;

    match result {
        Ok(user) => {
            match user.role <= role {
                true => Outcome::Success(T::default()),
                false => Outcome::Error((Status::Forbidden, "Erro no banco de dados"))
            }
        },
        Err(_) => Outcome::Error((Status::InternalServerError, "Erro interno"))
    }

}