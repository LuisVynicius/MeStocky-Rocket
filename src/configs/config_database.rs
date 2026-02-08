use sea_orm::{Database, DatabaseConnection};

use crate::configs::config_environment::get_database_url;

pub async fn get_database() -> DatabaseConnection {
    let database_url = get_database_url();

    let connection = Database::connect(database_url).await;

    match connection {
        Ok(database) => database,
        Err(_) => panic!("Não foi possível se conectar com o banco de dados"),
    }
}
