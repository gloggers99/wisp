#[macro_use] extern crate rocket;

use std::sync::{Arc, Mutex};
use beigebox_database::database::Database;

mod login;
mod signup;
mod home;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(
            Arc::new(
                Mutex::new(
                    // If this fails to open then we can't start the server anyway, so unwrapping
                    // is fine.
                    Database::open("db").unwrap()
                )
            )
        )
        .mount("/", routes![
            login::login_get,
            login::login_post,
            
            signup::signup_get,
            signup::signup_post,
            
            home::home_get
        ])
}