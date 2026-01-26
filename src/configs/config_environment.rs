use dotenv::Error;

pub fn get_database_url() -> String {

    let database_url = get_by_dotenv("DATABASE_URL");

    database_url
}

pub fn get_jwt_secret() -> String {

    let jwt_secret = get_by_dotenv("JWT_SECRET");

    jwt_secret

}

pub fn get_cost() -> u32 {

    let cost = get_by_dotenv("ENCRYPT_COST");

    match cost.parse::<u32>() {
        Ok(value) => {
            if value < 4 || value > 32 {
                panic!("O valor da variável da ENCRYPT_COST deve ser 4-32");
            }

            value
        },
        Err(_) => panic!("O valor da variável da ENCRYPT_COST deve ser um número")
    }
}

fn get_by_dotenv(value: &str) -> String {

    match dotenv::var(value) {
        Ok(value) => value,
        Err(err) => {
            match err {
                Error::EnvVar(_) => panic!("Não foi possível encontrar a variável de ambiente \"{value}\""),
                Error::Io(_) => panic!("Não foi possível acessar as variáveis de ambiente"),
                _ => panic!("Erro desconhecido ao acessar as variáveis de ambiente")
            }
        },
    }

}