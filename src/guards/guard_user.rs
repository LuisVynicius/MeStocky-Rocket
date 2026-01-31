use rocket::{http::Status, request::{FromRequest, Outcome, Request}};

use crate::configs::config_jwt::valid_token;

pub struct Authentication(pub String);

#[async_trait]
impl<'r> FromRequest<'r> for Authentication {
    
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        
        let token = req.headers().get_one("token");

        match token {
            Some(token) => match valid_token(token) {
                true => Outcome::Success(Authentication(token.to_string())),
                false => Outcome::Error((Status::Forbidden, "Token de autenticação inválido"))
            },
            None => Outcome::Error((Status::Forbidden, "Token de autenticação não encontrado, tente realizar o login"))
        }

    }

}