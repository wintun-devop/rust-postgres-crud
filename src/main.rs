mod config;
mod utils;
use sqlx::PgPool;
use utils::crud::{CreateItem, create_item};
use utils::hello::say_hello;
use utils::db::{init_db,get_pool};

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    println!("Welcome");
    say_hello(1).await;
    say_hello(2).await;
    let database_url = config::config().db_url;
    // initialize global pool once
    init_db(&database_url).await?;

    // obtain a clone of the pool when you need it
    let pool: PgPool = get_pool();

    let item_payload = CreateItem {
        name: "Widget103".to_string(),
        model: "X103".to_string(),
    };
    match create_item(item_payload, &pool).await {
        Ok(created) => println!("Created item:\n{:#?}", created),
        Err(e) => eprintln!("Failed to create item: {}", e),
    }
    Ok(())
}
