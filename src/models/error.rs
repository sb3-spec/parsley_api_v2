use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(sqlx::Error),

    #[error("{0}")]
    SqlxDbErr(String),

    #[error("Entity not found")]
    EntityNotFound,
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        if let Some(e) = err.as_database_error() {
            Error::SqlxDbErr(e.message().to_string())
        } else {
            Error::SqlxError(err)
        }
    }
}
