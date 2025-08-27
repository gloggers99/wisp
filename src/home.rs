use std::sync::{Arc, Mutex};

use maud::{Markup};

use rocket::response::Redirect;
use rocket::State;

use wisp_database::database::Database;

use wisp_session_manager::authenticated_user::AuthenticatedUser;

use wisp_pages::home::home_page;

#[get("/home")]
pub fn home_get(user: AuthenticatedUser,
                database: &State<Arc<Mutex<Database>>>) -> Markup {
    home_page(user.user)
}

#[get("/home", rank = 2)]
pub fn home_get_redirect() -> Redirect {
    Redirect::to(uri!("/login"))
}