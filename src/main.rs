use crate::{configs::config_cors::make_cors, routes::{route_category::{route_category_create, route_category_delete, route_category_get_all, route_category_get_all_admin, route_category_update}, route_product::{route_product_create, route_product_delete, route_product_get_all, route_product_informations, route_product_quantity_update, route_product_update}, route_reason::{route_reason_create, route_reason_delete, route_reason_get_all, route_reason_update}, route_report::route_report_get_all, route_user::{route_login, route_user_create, route_user_delete, route_user_get_all, route_user_role_update, route_user_update_credentials, route_user_update_informations, route_valid_token}}};

#[macro_use] extern crate rocket;

mod routes;
mod entities;
mod configs;
mod services;
mod guards;
mod errors;

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
                route_valid_token,
                route_user_get_all,
                route_user_create,
                route_user_update_informations,
                route_user_update_credentials,
                route_user_role_update,
                route_user_delete,

                route_category_get_all,
                route_category_get_all_admin,
                route_category_create,
                route_category_update,
                route_category_delete,

                route_product_get_all,
                route_product_informations,
                route_product_create,
                route_product_update,
                route_product_delete,
                route_product_quantity_update,

                route_reason_get_all,
                route_reason_create,
                route_reason_update,
                route_reason_delete,

                route_report_get_all
            ]
        )
}