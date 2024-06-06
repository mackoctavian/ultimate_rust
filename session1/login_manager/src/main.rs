use std::collections::HashMap;

use clap::{Parser, Subcommand};
use authentication::{get_users, save_users, LoginRole, User};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ///List all users.
    List,
    //Add a users
    Add {
        //The user's login name
        username: String,

        //The user's password (plaintext)
        password: String,

        //Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>
    },

    //Delete user
    Delete {
        // User to delete
        username: String
    },

    //Change a user's password 
    ChangePassword {
        //Username that needs a password change
        username: String,

        //new pasword
        new_password: String,
    }
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users: HashMap<String, User> = get_users();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };

    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

//listing users
fn list_users() {
    println!("{:<20}{:<20}",  "username", "Role");
    println!("{:-<40}", "");

    let users  = get_users();
    users.iter().for_each(|(_, user)| {
        println!("{:<20} {:20?}", user.username, user.role);
    });
}

fn delete_user(username: String) {
    let mut users: HashMap<String, User> = get_users();

    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
    }else {
        println!("{username} does not exist")
    }
}

fn change_password(username: String, password: String) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(&username) {

        user.password = authentication::hash_password(& password);
        save_users(users);

    }else {
        println!("{username} does not exist")
    }
}

fn main() {
    let cli = Args::parse();

    match cli.command{
        Some(Commands::List) => {
            list_users();
        }

        Some(Commands::Add { username, password, admin }) => {
            add_user(username, password, admin.unwrap_or(false));
        }

        Some(Commands::Delete { username }) => {
            delete_user(username);
        }

        Some(Commands::ChangePassword { username, new_password }) => {
            change_password(username, new_password);
        }

        None => {
            println!("Run with --help to see instructions")
        } 
    }
}
