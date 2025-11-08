mod utils;
use utils::hello::say_hello;

// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
#[tokio::main]
async fn main() {
    println!("Welcome");
    say_hello(1).await;
    say_hello(2).await;
}
