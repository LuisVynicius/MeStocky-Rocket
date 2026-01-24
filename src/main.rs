use crate::{configs::config_jwt::valid_token, routes::{route_role::{route_role_create, route_role_get_all}, route_user::{route_login, route_user_create, route_user_get_all, route_user_role_update, route_user_update}}};

#[macro_use] extern crate rocket;

mod routes;
mod entities;
mod configs;
mod services;
mod guards;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(
            configs::config_database::get_database().await
        )
        .mount(
            "/", routes![
                route_login,
                route_user_get_all,
                route_user_create,
                route_user_update,
                route_user_role_update,

                route_role_get_all,
                route_role_create,

                test
            ]
        )
}

#[get("/test/<token>")]
async fn test(token: String) {
    println!("Token: {}", valid_token(token));
}