mod config;
mod utils;
use sqlx::PgPool;
use utils::crud::{CreateItem, create_item};
use utils::hello::say_hello;

// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    println!("Welcome");
    say_hello(1).await;
    say_hello(2).await;
    let database_url = config::config().db_url;

    let pool: PgPool = PgPool::connect(&database_url).await?;

    let item_payload = CreateItem {
        name: "Widget101".to_string(),
        model: "X101".to_string(),
    };
    match create_item(item_payload, &pool).await {
        Ok(created) => println!("Created item:\n{:#?}", created),
        Err(e) => eprintln!("Failed to create item: {}", e),
    }
    Ok(())
}
