use std::collections::HashMap;

pub fn greet_user(name: &str) {
    println!("Hello {name}")
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let users = get_users();

    if let Some(user) = users.get(&username) {
        if user.password == password {
           return  Some(LoginAction::Granted(user.role.clone()));
        }else {
            return  Some(LoginAction::Denied);
        }
    }

    None
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    role: LoginRole
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> Self {
        Self {
            username: username.to_lowercase(),
            password: password.to_string(),
            role
        }
    }
}

// pub fn get_users() -> Vec<User>{
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("user", "password", LoginRole::User)
//     ]
// }

fn  get_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin),);
    users.insert("user".to_string(), User::new("user", "password", LoginRole::User));
    users
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_login() {
       assert_eq!(login("admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
       assert_eq!(login("user", "password"), Some(LoginAction::Granted(LoginRole::User)));
       assert_eq!(login("admin", "not-password"), Some(LoginAction::Denied));
    }

}
