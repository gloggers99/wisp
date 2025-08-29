use std::sync::{Arc, Mutex};

use maud::{html, Markup};
use rocket::http::StatusClass::ServerError;
use rocket::response::{Flash, Redirect};
use rocket::response::status::NotFound;
use rocket::State;

use wisp_database::database::Database;

use wisp_session_manager::admin_user::AdminUser;

use wisp_pages::admin::admin_page;

#[get("/admin")]
pub fn admin_get(user: AdminUser,
                 database: &State<Arc<Mutex<Database>>>) -> Markup {
    if let Ok(database) = database.lock() {
        admin_page(&user, &database)
    } else {
        html!( "Internal server error" )
    }
}

#[get("/admin", rank = 2)]
pub fn admin_redirect() -> &'static str {
    "You are not permitted to enter this page."
}