use rocket::serde::json::Json;

use crate::entities::dtos::login_dtos::LoginDTO;

#[post("/login", data="<logindto>")]
pub fn route_login(logindto: Json<LoginDTO>) -> String {
    println!("{} - {}", logindto.get_username(), logindto.get_password());
    String::new()
}