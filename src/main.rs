mod utils;
mod config;
use utils::hello::say_hello;
use utils::db::init_db;
use utils::crud::create_user;


// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
#[tokio::main]
async fn main() {
    println!("Welcome");
    say_hello(1).await;
    say_hello(2).await;
    let my_db = init_db().await;
     match create_user(my_db).await {
        Ok(json_value) => {
            println!("create_user returned: {}", json_value);
        }
        Err(err) => {
            eprintln!("create_user failed: {:?}", err);
        }
    }
}
