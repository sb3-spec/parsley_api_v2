use db::connect_to_db;
use dotenvy;
use models::ChefPatch;
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
    let db = connect_to_db().await.expect("Cannot connect to db");

    let _chef_data = ChefPatch {
        username: Some("test".to_string()),
        custom_tags: Some(Vec::new()),
        email: Some("test@example.com".to_string()),
    };

    let _db = Arc::new(db);
}
