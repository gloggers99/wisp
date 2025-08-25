use crate::user::User;

use std::error::Error;
use std::path::Path;

use sled::Db;

use bincode;
use bincode::config::standard;

/// Indicates the method to search for a user in the database.
pub enum UserQuery<'a> {
    Username(&'a str)
}

/// This is a beigebox specific wrapper around sled.
pub struct Database {
    internal_database: Db
} impl Database {
    pub fn open<T: AsRef<Path>>(path: T) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            internal_database: sled::open(path)?
        })
    }
    
    pub fn get_all_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        self.internal_database
            .iter()
            .map(|pair| {
                let (_, value) = pair?;
                let (user, _) = bincode::decode_from_slice(&value, standard())?;
                Ok::<User, Box<dyn Error>>(user)
            })
            .collect()
    }
    
    pub fn get_user(&self, query: UserQuery) -> Result<Option<User>, Box<dyn Error>> {
        match query { 
            UserQuery::Username(username) => {
                if let Some(ivec) = self.internal_database.get(username)? {
                    let (user, _) = bincode::decode_from_slice(&ivec, standard())?;
                    Ok(Some(user))
                } else { 
                    Ok(None)
                }
            }
        }
    }
    
    pub fn add_user(&self, user: User) -> Result<(), Box<dyn Error>> {
        // Check if user already exists.
        if let Ok(Some(_)) = self.get_user(UserQuery::Username(user.username())) {
            Err(String::from("User already exists."))?
        }
        
        // Check username for bad stuff & formatting.
        if user.username().contains(|c: char| !c.is_ascii_alphanumeric()) {
            Err(String::from("Username can only contain letters and numbers."))?
        }
        
        // User doesn't exist so we can add it to our database.
        let user_encoded = bincode::encode_to_vec(&user, standard())?;
        self.internal_database.insert(user.username(), user_encoded)?;
        self.internal_database.flush()?;
        
        Ok(())
    }
}