use crate::configs::config_environment::get_cost;

pub fn encrypt_password(password: &str) -> String {
    let cost = get_cost();

    bcrypt::hash(password, cost).unwrap()
}

pub fn verify_password(password: &str, encrypted_password: &str) -> bool {
    bcrypt::verify(password, encrypted_password).unwrap()
}
