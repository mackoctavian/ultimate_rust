use tokio::runtime;

async fn hello() {
    println!("Hello Tokio")
}
fn main() {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(hello());
}
