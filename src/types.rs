// src/types.rs

#[derive(Clone, Debug)]
pub struct Message {
    pub username: String,
    pub content: String,
}

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    // other fields...
}
