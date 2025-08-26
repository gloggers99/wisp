use bincode::{Decode, Encode};

// We should figure out if we can somehow do this without the `FromForm` macro.
use rocket::form::FromForm;

/// Represents a user on the database
#[derive(Encode, Decode, FromForm)]
pub struct User {
    email: String,
    username: String,
    password: String,
} impl User {
    pub fn new(email: String, username: String, password: String) -> Self {
        Self {
            email,
            username,
            password
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
