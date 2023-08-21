use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Failed to connect to database")]
    DatabaseConnectionFailed(#[from] sqlx::Error),

    #[error("Failed to read environment variable")]
    EnvironmentVariableReadFailure(#[from] std::env::VarError),

    #[error("Failed to read sql files")]
    SqlFileReadFailure(#[from] std::io::Error),

    #[error("Failed to apply migrations")]
    MigrationFailed(#[from] sqlx::migrate::MigrateError),
}
