#[macro_use] extern crate rocket;

use crate::{configs::get_database, routes::login::route_login};

mod routes;
mod entities;
mod configs;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .manage(
            get_database().await
        )
        .mount(
            "/", routes![
                route_login
            ]
        )
}