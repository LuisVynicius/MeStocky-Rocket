pub enum UserRole {
    Admin,
    Mannager,
    Operator,
    Viewer
}

impl UserRole {

    pub fn to_code(user_role: UserRole) -> u8 {

        match user_role {
            Self::Admin => 1,
            Self::Mannager => 2,
            Self::Operator => 3,
            Self::Viewer => 4
        }

    }

    pub fn code_to_string(user_role: u8) -> String {

        match user_role {
            1 => String::from("Administrador"),
            2 => String::from("Gerente"),
            3 => String::from("Operador"),
            _ => String::from("Visualizador"),
        }

    }

}