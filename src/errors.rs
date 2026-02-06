use sea_orm::DbErr;

pub enum BackendError {
    DatabaseError(DbErr),

    ResourceAlreadyInsertedError,
    ResourceNotFoundError,
    ResourceConflitUpdateError,

    InvalidCredentialsError,

    InvalidValuesError
}