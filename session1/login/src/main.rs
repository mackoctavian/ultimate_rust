use authentication::{greet_user, login, read_line, LoginAction}; 

fn main() {
    let mut tries = 0;

    loop { 
        println!("Enter your username: ");
        let username = read_line();
        println!("Enter your password");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(role)) => {
                match role {
                    authentication::LoginRole::Admin => println!("Admin"),
                    authentication::LoginRole::User => println!("user")
                }

                break;
            }

            Some(LoginAction::Denied) => { println!("Denied") }

            None => { 
                println!("New user in our system");
                break;
        }
        }

        
    }
}
