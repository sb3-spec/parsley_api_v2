use sqlx::PgPool;
use thiserror::Error as ThisError;

pub struct UserCtx {
    pub user_id: String,
}

pub async fn utx_from_token(_db: &PgPool, token: &str) -> Result<UserCtx, Error> {
    match token.parse::<String>() {
        Ok(user_id) => Ok(UserCtx { user_id }),
        Err(_) => Err(Error::InvalidToken(token.to_string())),
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Invalid Token {0}")]
    InvalidToken(String),
}
