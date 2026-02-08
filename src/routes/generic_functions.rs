use rocket::{http::Status, response::status::Custom};
use sea_orm::DbErr;

use crate::errors::BackendError;

pub fn catch_backend_error(backend_error: BackendError) -> Custom<&'static str> {
    match backend_error {
        BackendError::DatabaseError(db_err) => match db_err {
            DbErr::AttrNotSet(_)
            | DbErr::ConvertFromU64(_)
            | DbErr::Type(_)
            | DbErr::Json(_)
            | DbErr::Custom(_) => Custom(Status::UnprocessableEntity, ""),

            _ => Custom(Status::InternalServerError, "Erro interno do sistema"),
        },

        BackendError::ResourceAlreadyInsertedError => Custom(
            Status::Conflict,
            "Uma entidade com o identificador desejado já existe",
        ),
        BackendError::ResourceNotFoundError => Custom(Status::NotFound, "Entidade não encontrada"),
        BackendError::ResourceConflitUpdateError => Custom(
            Status::Conflict,
            "A entidade possuí identificadores presentes em outras entidades já salvas",
        ),

        BackendError::InvalidCredentialsError => {
            Custom(Status::Forbidden, "As credenciais inserídas são inválidas")
        }
        BackendError::InvalidValuesError => Custom(
            Status::BadRequest,
            "Um ou mais valores enviados são inválidas",
        ),
    }
}
