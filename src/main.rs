use db::connect_to_db;
use dotenvy;
use recipe_scraper::scrape_recipe;
use std::{env, sync::Arc};

mod auth;
mod db;
mod models;
mod recipe_scraper;
// mod routes;

const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    match dotenvy::dotenv() {
        Ok(_) => println!("Dev vars successfully loaded"),
        Err(_) => println!("Failed to load dev vars"),
    };

    let _web_port: u16 = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_) => DEFAULT_WEB_PORT,
    };

    // Connect to database

    match connect_to_db().await {
        Ok(_) => println!("DB successfully connected"),
        Err(_) => println!(
            "Failed to connect to DB at url: {}",
            env::var("DATABASE_URL").unwrap()
        ),
    };
    let _db = Arc::new(connect_to_db().await.expect("Cannot connect to db"));

    match scrape_recipe(
        "https%3A%2F%2Fwww.allrecipes.com%2Frecipe%2F269276%2Fsimple-tomato-soup%2F",
    )
    .await
    {
        Ok(recipe) => {
            println!("Got recipe: {:?}", recipe);
        }
        Err(e) => {
            println!("Got error: {:?}", e);
        }
    };
}
