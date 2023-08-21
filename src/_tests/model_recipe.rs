use crate::{
    auth::utx_from_token,
    db::connect_to_test_db,
    models::{RecipeMac, RecipePatch},
};

/// Tests create and delete
#[tokio::test]
async fn model_recipe_create_and_delete() -> Result<(), Box<dyn std::error::Error>> {
    match dotenvy::dotenv() {
        Ok(_) => println!("Dev vars successfully loaded"),
        Err(_) => println!("Failed to load dev vars"),
    };
    let db = connect_to_test_db().await?;

    let data: RecipePatch = RecipePatch {
        title: Some("Test".to_string()),
        header: Some("Test".to_string()),
        ingredients: Some(vec!["Test".to_string(), "Test".to_string()]),
        steps: Some(vec!["Test".to_string(), "Test".to_string()]),
        tags: Some(vec!["Test".to_string(), "Test".to_string()]),
        image_url: Some("Test".to_string()),
        cook_time: Some("Test".to_string()),
        prep_time: Some("Test".to_string()),
        total_time: Some("Test".to_string()),
    };

    let user_ctx = utx_from_token(&db, "123").await?;

    let new_recipe = RecipeMac::create(&db, data, user_ctx).await?;

    let user_ctx = utx_from_token(&db, "123").await?;

    RecipeMac::delete(&db, user_ctx, new_recipe.id).await?;

    Ok(())
}

#[tokio::test]
async fn model_recipe_list() -> Result<(), Box<dyn std::error::Error>> {
    match dotenvy::dotenv() {
        Ok(_) => println!("Dev vars successfully loaded"),
        Err(_) => println!("Failed to load dev vars"),
    };

    let db = connect_to_test_db().await?;

    let user_ctx = utx_from_token(&db, "123").await?;

    RecipeMac::list(&db, user_ctx).await?;

    Ok(())
}

#[tokio::test]
async fn model_recipe_get() -> Result<(), Box<dyn std::error::Error>> {
    match dotenvy::dotenv() {
        Ok(_) => println!("Dev vars successfully loaded"),
        Err(_) => println!("Failed to load dev vars"),
    };

    let db = connect_to_test_db().await?;

    let data: RecipePatch = RecipePatch {
        title: Some("Test".to_string()),
        header: Some("Test".to_string()),
        ingredients: Some(vec!["Test".to_string(), "Test".to_string()]),
        steps: Some(vec!["Test".to_string(), "Test".to_string()]),
        tags: Some(vec!["Test".to_string(), "Test".to_string()]),
        image_url: Some("Test".to_string()),
        cook_time: Some("Test".to_string()),
        prep_time: Some("Test".to_string()),
        total_time: Some("Test".to_string()),
    };

    let user_ctx = utx_from_token(&db, "123").await?;
    let new_recipe = RecipeMac::create(&db, data, user_ctx).await?;

    let user_ctx = utx_from_token(&db, "123").await?;

    let result = RecipeMac::get(&db, user_ctx, new_recipe.id).await?;
    assert_eq!(result.id, new_recipe.id);

    let user_ctx = utx_from_token(&db, "123").await?;

    RecipeMac::delete(&db, user_ctx, new_recipe.id).await?;

    Ok(())
}

// #[tokio::test]
// async fn model_recipe_delete() -> Result<(), Box<dyn std::error::Error>> {
//     match dotenvy::dotenv() {
//         Ok(_) => println!("Dev vars successfully loaded"),
//         Err(_) => println!("Failed to load dev vars"),
//     };

//     let db = connect_to_test_db().await?;

//     let user_ctx = utx_from_token(&db, "123").await?;

//     RecipeMac::delete(&db, user_ctx, 1).await?;

//     Ok(())
// }
