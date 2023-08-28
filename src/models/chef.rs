use super::error::Error;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth::UserCtx;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ChefPatch {
    pub username: Option<String>,
    pub custom_tags: Option<Vec<String>>,
    pub email: Option<String>,
}

pub struct ChefMac;

impl ChefMac {
    pub async fn create(db: &PgPool, data: ChefPatch, user_ctx: UserCtx) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO chef (auth_id, username, custom_tags, email)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(user_ctx.user_id)
        .bind(
            &data
                .username
                .unwrap_or_else(|| String::from("Default Chef")),
        )
        .bind::<Vec<String>>(Vec::new())
        .bind(&data.email.unwrap_or_default())
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(db: &PgPool, user_ctx: UserCtx) -> Result<(), Error> {
        sqlx::query!("DELETE FROM chef WHERE auth_id = $1", user_ctx.user_id)
            .execute(db)
            .await?;

        Ok(())
    }
}
