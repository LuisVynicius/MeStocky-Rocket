pub enum UserRole {
    Admin,
    Mannager,
    Operator,
    Viewer,
}

impl UserRole {
    pub fn code_to_string(user_role: u8) -> String {
        match user_role {
            1 => String::from("Administrador"),
            2 => String::from("Gerente"),
            3 => String::from("Operador"),
            _ => String::from("Visualizador"),
        }
    }
}
