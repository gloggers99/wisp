use std::collections::HashMap;
use bincode::{Decode, Encode};
use uuid::Uuid;

#[derive(Encode, Decode)]
pub struct Post {
    title: String,
    content: String
}

/// Represents a user on the database
#[derive(Encode, Decode)]
pub struct User {
    email: String,
    username: String,
    password: String,
    posts: HashMap<i128, Post>
} impl User {
    pub fn new(email: String, 
               username: String, 
               password: String, 
               posts: Option<HashMap<i128, Post>>) -> Self {
        Self {
            email,
            username,
            password,
            posts: posts.unwrap_or_else(|| HashMap::new())
        }
    }
    
    pub fn email(&self) -> &str {
        &self.email
    }
    
    pub fn username(&self) -> &str {
        &self.username
    }
    
    pub fn password(&self) -> &str {
        &self.password
    }
}
