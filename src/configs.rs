use dotenv::Error;
use sea_orm::{Database, DatabaseConnection};

pub async fn get_database() -> DatabaseConnection {
    match get_database_url() {
        Ok(database_url) => {
            let connection = Database::connect(database_url).await;
            
            match connection {
                Ok(database) => database,
                Err(_) => panic!("Não foi possível se conectar com o banco de dados")
            }
        },
        Err(_) => panic!("Não foi possível acessar a varíavel de ambiente DATABASE_URL")
    }
}

fn get_database_url() -> Result<String, Error> {
    Ok(dotenv::var("DATABASE_URL")?)
}