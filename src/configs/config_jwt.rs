use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};
use serde::{Deserialize, Serialize};

use crate::configs::config_environment::get_jwt_secret;

pub fn generate_token(email: String) -> Result<String, Error> {

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 14400;

    let claim = Claim {
        sub: email,
        exp: expiration as u32
    };

    let jwt_secret = get_jwt_secret();

    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(
            jwt_secret.as_bytes()
        )
    );

    Ok(token?)

}

pub fn valid_token(token: String) -> bool {

    let jwt_secret = get_jwt_secret();
    
    let claim = decode::<Claim>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default()
    );

    claim.is_ok()
    
}

pub fn get_email_by_token(token: String) -> String {

    let jwt_secret = get_jwt_secret();
    
    let claim = decode::<Claim>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default()
    );

    claim.unwrap().claims.sub

}

#[derive(Debug, Serialize, Deserialize)]
struct Claim {
    sub: String,
    exp: u32
}

