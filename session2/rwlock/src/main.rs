use std::{io, sync::RwLock, thread, time::Duration};
use once_cell::sync::Lazy;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));
fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current users (in a thread) ");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            thread::sleep(Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter a name too add to users list (or q to quit)");
        let input: String = read_line();
        if input == "q" {
            break;
        }else {
            let mut lock = USERS.write().unwrap();
            lock.push(input);
        }
    }
    
}


fn build_users() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

fn read_line() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}