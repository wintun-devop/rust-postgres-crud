use tokio::time::{sleep, Duration};

pub async fn say_hello(num:i8) {
    // async function can do awaits inside
    println!("Hello from async function! {}",num);
    sleep(Duration::from_millis(200)).await;
    println!("Goodbye from async function!");
}