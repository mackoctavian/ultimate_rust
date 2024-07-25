async fn hello() {
    println!("Hello Tokio")
}

#[tokio::main]
async fn main() {
    hello().await;
}
