use crate::{configs::{config_cors::make_cors, config_jwt::valid_token}, routes::{route_category::{route_category_create, route_category_get_all, route_category_update}, route_product::{route_product_create, route_product_get_all, route_product_informations, route_product_quantity_update, route_product_update}, route_report::route_report_get_all, route_return_reason::{route_return_reason_create, route_return_reason_get_all, route_return_reason_update}, route_user::{route_login, route_user_create, route_user_get_all, route_user_role_update, route_user_update_informations}}};

#[macro_use] extern crate rocket;

mod routes;
mod entities;
mod configs;
mod services;
mod guards;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(make_cors())
        .manage(
            configs::config_database::get_database().await
        )
        .mount(
            "/", routes![
                route_login,
                route_user_get_all,
                route_user_create,
                route_user_update_informations,
                route_user_role_update,

                route_category_get_all,
                route_category_create,
                route_category_update,

                route_product_get_all,
                route_product_informations,
                route_product_create,
                route_product_update,
                route_product_quantity_update,

                route_return_reason_get_all,
                route_return_reason_create,
                route_return_reason_update,

                route_report_get_all,

                test
            ]
        )
}

#[get("/test/<token>")]
async fn test(token: String) {
    println!("Token: {}", valid_token(token));
}