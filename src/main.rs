#[macro_use] extern crate rocket;

use std::sync::{Arc, Mutex};

use beigebox_database::database::Database;
use beigebox_session_manager::session_manager::SessionManager;

mod login;
mod signup;
mod home;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Arc::new(Mutex::new(Database::open("db").unwrap())))
        .manage(Arc::new(Mutex::new(SessionManager::new())))
        .mount("/", routes![
            login::login_get,
            login::login_post,
            
            signup::signup_get,
            signup::signup_post,
            
            home::home_get,
            home::home_get_redirect
        ])
}