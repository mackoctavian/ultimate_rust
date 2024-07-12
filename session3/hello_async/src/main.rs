use std::thread;
use std::time::Duration;

use futures::executor::block_on;
use futures::join;

fn main() {
    let future = say_hello();
    block_on(future);
}

async fn say_hello() {
    println!("Hello");
    join!(sec_function(), say_goodbye());
}

async fn say_goodbye() {
    println!("Goodbye");
}

async fn sec_function() {
    thread::sleep(Duration::from_secs(3));
    println!("Hello Again");
}
