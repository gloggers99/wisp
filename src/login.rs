use std::sync::{Arc, Mutex};

use maud::{Markup};

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::State;

use wisp_database::database::{Database, UserQuery};

use wisp_session_manager::session_manager::SessionManager;

use wisp_pages::login::login_page;

#[derive(FromForm)]
pub struct LoginForm {
    username: String,
    password: String
}

/// The login page the user is faced with.
/// 
/// This will send `LoginForm` as a post request to the server.
#[get("/login")]
pub fn login_get(flash: Option<FlashMessage>) -> Markup {
    if let Some(flash) = flash {
        login_page(Some(flash.message()))
    } else {
        login_page(None)
    }
}

#[post("/login", data = "<form_data>")]
pub fn login_post(form_data: Form<LoginForm>, 
                  database: &State<Arc<Mutex<Database>>>,
                  session_manager: &State<Arc<Mutex<SessionManager>>>,
                  cookie_jar: &CookieJar) -> Result<Redirect, Flash<Redirect>> {
    let form_data = form_data.into_inner();

    if let Ok(database) = database.lock()
        && let Ok(Some(user)) = database.get_user(UserQuery::Username(&form_data.username))
        && user.password() == form_data.password
        && let Ok(session_manager) = session_manager.lock() {
        
        cookie_jar.add_private(
            Cookie::build(("session_id", session_manager.generate_session(user.username()).uuid().to_string()))
                .path("/")
                .http_only(true)
        );

        Ok(Redirect::to("/home"))
    } else {
        Err(Flash::error(Redirect::to("/login"), "Invalid credentials."))
    } 
}