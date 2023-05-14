use std::collections::HashMap;
use bcrypt::{hash, verify, DEFAULT_COST};

pub struct User{
    pub username: String,
    pub password_hash: String,
}

pub type Database = HashMap<String, User>;

pub fn initialize_database()=> Database{

    let mut database = HashMap::new();
    let password_hash = hash("password", DEFAULT_COST).unwrap();
    database.insert("user1", to_string(), User{username: "user1".to_string(), password_hash});
    database
}

pub fn authenticate_user(username: &str, password: &str, database: &Database) -> Option<&User>{
    if let Some(user) = database.get(username){
        if verify(password, &user.password_hash).unwrap(){
            return Some(user);
        }
    }
    None
}
