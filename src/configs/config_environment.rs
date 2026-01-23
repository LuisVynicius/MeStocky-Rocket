use dotenv::Error;

pub fn get_database_url() -> String {

    match dotenv::var("DATABASE_URL") {
        Ok(database_url) => database_url,
        Err(err) => {
            match err {
                Error::EnvVar(_) => panic!("Não foi possível encontrar a variável de ambiente \"DATABASE_URL\""),
                Error::Io(_) => panic!("Não foi possível acessar as variáveis de ambiente"),
                _ => panic!("Erro desconhecido ao acessar as variáveis de ambiente")
            }
        },
    }

}

pub fn get_jwt_secret() -> String {

    match dotenv::var("JWT_SECRET") {
        Ok(jwt_secret) => jwt_secret,
        Err(err) => {
            match err {
                Error::EnvVar(_) => panic!("Não foi possível encontrar a variável de ambiente \"JWT_SECRET\""),
                Error::Io(_) => panic!("Não foi possível acessar as variáveis de ambiente"),
                _ => panic!("Erro desconhecido ao acessar as variáveis de ambiente")
            }
        },
    }

}