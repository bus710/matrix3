mod client;

use std::collections::HashMap;
use tokio::time::{sleep, Duration};


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    client::random().await;
    sleep(Duration::from_secs(2)).await;
    client::all_1().await;
    sleep(Duration::from_secs(2)).await;

    println!("Bye")
}
