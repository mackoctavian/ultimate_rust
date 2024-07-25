use futures::executor::block_on;
use futures::future::{join, join_all};
use futures::join;

fn main() {
    block_on(say_hello());
}

async fn say_hello() {
    println!("Hello");
    join!(sec_function(), say_goodbye());

    let n = double(2).await;
    println!("{n}");

    let futures = vec![double(1), double(2)];
    let result = join_all(futures).await;

    do_something()
}

async fn sec_function() {
    println!("Hello Again")
}

async fn say_goodbye() {
    println!("Say Goobye")
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn do_something() {
    println!("Not async");
}
