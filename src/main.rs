mod utils;
mod config;
use utils::hello::say_hello;
use utils::db::init_db;


// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
#[tokio::main]
async fn main() {
    println!("Welcome");
    say_hello(1).await;
    say_hello(2).await;
    let _my_db = init_db().await;
}
