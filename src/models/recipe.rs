use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::auth::UserCtx;

use super::error::Error;

#[derive(Debug, FromRow)]
pub struct Recipe {
    pub id: i64,
    pub cid: String,
    pub ctime: sqlx::types::time::OffsetDateTime,
    pub mtime: sqlx::types::time::OffsetDateTime,
    pub title: String,
    pub header: String,
    pub ingredients: Vec<String>,
    pub steps: Vec<String>,
    pub tags: Vec<String>,
    pub image_url: String,
    pub cook_time: String,
    pub prep_time: String,
    pub total_time: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct RecipePatch {
    pub title: Option<String>,
    pub header: Option<String>,
    pub steps: Option<Vec<String>>,
    pub ingredients: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub image_url: Option<String>,
    pub cook_time: Option<String>,
    pub prep_time: Option<String>,
    pub total_time: Option<String>,
}

pub struct RecipeMac;

#[allow(dead_code)]
impl RecipeMac {
    pub async fn create(
        db: &PgPool,
        data: RecipePatch,
        user_ctx: UserCtx,
    ) -> Result<Recipe, Error> {
        let query = "INSERT INTO recipe (cid, title, header, steps, ingredients, tags, image_url, cook_time, prep_time, total_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) returning *";

        let recipe = sqlx::query_as::<_, Recipe>(query)
            .bind(user_ctx.user_id)
            .bind(&data.title.unwrap_or_default())
            .bind(&data.header.unwrap_or_default())
            .bind(&data.steps.unwrap_or_default())
            .bind(&data.ingredients.unwrap_or_default())
            .bind(&data.tags.unwrap_or_default())
            .bind(&data.image_url.unwrap_or_default())
            .bind(&data.cook_time.unwrap_or_default())
            .bind(&data.prep_time.unwrap_or_default())
            .bind(&data.total_time.unwrap_or_default())
            .fetch_one(db)
            .await?;

        Ok(recipe)
    }

    pub async fn list(db: &PgPool, user_ctx: UserCtx) -> Result<Vec<Recipe>, Error> {
        let query = "SELECT * FROM recipe WHERE cid = $1 ORDER BY ctime DESC";

        let data = sqlx::query_as::<_, Recipe>(query)
            .bind(user_ctx.user_id)
            .fetch_all(db)
            .await?;

        Ok(data)
    }

    pub async fn delete(db: &PgPool, _user_ctx: UserCtx, target_id: i64) -> Result<(), Error> {
        let query = "DELETE FROM recipe WHERE id = $1";

        sqlx::query(query).bind(target_id).execute(db).await?;

        Ok(())
    }

    pub async fn get(db: &PgPool, _user_ctx: UserCtx, target_id: i64) -> Result<Recipe, Error> {
        let query = "SELECT * FROM recipe WHERE id = $1";

        let query_result = sqlx::query_as::<_, Recipe>(query)
            .bind(target_id)
            .fetch_one(db)
            .await?;

        Ok(query_result)
    }
}

#[cfg(test)]
#[path = "../_tests/model_recipe.rs"]
mod tests;
