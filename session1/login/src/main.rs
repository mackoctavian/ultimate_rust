use authentication::{login, read_line, greet_user}; 

fn main() {
    let mut tries = 0;

    loop { 
        println!("Enter your username: ");
        let username = read_line();
        println!("Enter your password");
        let password = read_line();

        if login(&username, &password) {
            greet_user(&username);
            break;
        }else {
            println!("Incorrect username of password");
            tries += 1;

            if tries >= 3 { 
                println!("Too many tries");
                break;
            }else {
                let tries_remaining = 3 - tries;  
                println!("{} trial remaining", tries_remaining);
            }
        }
    }
}
