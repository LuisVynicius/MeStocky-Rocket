use crate::{configs::config_jwt::valid_token, routes::route_user::{route_login, route_user_create}};

#[macro_use] extern crate rocket;

mod routes;
mod entities;
mod configs;
mod services;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(
            configs::config_database::get_database().await
        )
        .mount(
            "/", routes![
                route_login,
                route_user_create,
                test
            ]
        )
}

#[get("/test/<token>")]
async fn test(token: String) {
    println!("Token: {}", valid_token(token));
}