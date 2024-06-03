use std::{collections::HashMap, path::Path};
use serde::{Serialize, Deserialize};

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let users = get_users();
    let password = hash_password(password);

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

#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Stdin not working");
    input.trim().to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    role: LoginRole
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> Self {
        Self {
            username: username.to_lowercase(),
            password: hash_password(password),
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
    let users_path = Path::new("users.json");
    if users_path.exists() {
        //Load File
        let user_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&user_json).unwrap();
        users

    }else {
        //Create a file
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

fn  get_default_users() -> HashMap<String, User> {
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
