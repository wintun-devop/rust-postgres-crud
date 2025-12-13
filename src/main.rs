mod config;
mod utils;
use utils::crud::{CreateItem,UpdateItem, create_item, get_all_items, update_item};
use utils::hello::say_hello;

use crate::utils::db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Welcome");
    say_hello(1).await;
    say_hello(2).await;
    let pool = db::get_pool().await?;
    let item_payload = CreateItem {
        name: "Widget202".to_string(),
        model: "X202".to_string(),
    };
    match create_item(item_payload, &pool).await {
        Ok(created) => println!("Created item:\n{:#?}", created),
        Err(e) => eprintln!("Failed to create item: {}", e),
    }
    // list
    let all = get_all_items(&pool).await?;
    println!("{:#?}", all);
    // update (partial)
    let _updated = update_item(
        "273e1a14-5bdc-4df8-be84-2ab18cdc553d",
        UpdateItem {
            name: "Widget104".to_string(),
            model: "X104".to_string(),
        },
        &pool,
    )
    .await?;
    // delete
    // let deleted = delete_item(&created.id, &pool).await?;

    Ok(())
}
