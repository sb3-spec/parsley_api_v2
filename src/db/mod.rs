use sqlx::postgres::PgPool;
use std::env;

mod error;

use error::Error;

pub async fn connect_to_db() -> Result<PgPool, Error> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

#[allow(dead_code)]
pub async fn connect_to_test_db() -> Result<PgPool, Error> {
    let pool = PgPool::connect(&env::var("TEST_DATABASE_URL")?).await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
