use std::sync::{Arc, Mutex};

use maud::{html, Markup};

use rocket::response::Redirect;
use rocket::State;

use wisp_database::database::Database;

use wisp_session_manager::authenticated_user::AuthenticatedUser;

#[get("/home")]
pub fn home_get(user: AuthenticatedUser,
                database: &State<Arc<Mutex<Database>>>) -> Markup {
    let database = database.lock().unwrap();
    let all_users = database.get_all_users().unwrap();
    
    html!(
        head {
            link rel="stylesheet" href="/static/global_style.css" {}
        }
        
        body {
            h1 { "Welcome, " (user.user.username()) "!" }
            @for user in all_users {
                p{ (user.email()) " " (user.username()) " " (user.password()) }
            }
        }
    )
}

#[get("/home", rank = 2)]
pub fn home_get_redirect() -> Redirect {
    Redirect::to(uri!("/login"))
}