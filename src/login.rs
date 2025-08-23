use std::sync::{Arc, Mutex};

use maud::{html, Markup};

use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::State;

use beigebox_database::database::{Database, UserQuery};

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
    html!(
        (maud::DOCTYPE)
        body {
            @if let Some(flash) = flash {
                p style="color:red; font-weight: bold;" { (flash.message()) }
            }
            form method="post" action="/login" {
                label for="username" { "Username:" }
                input type="text" name="username" id="username" required;
                br;
                label for="password" { "Password:" }
                input type="password" name="password" id="password" required;
                br;
                button type="submit" { "Log In" }
            }
        }
    )
}

#[post("/login", data = "<form_data>")]
pub fn login_post(form_data: Form<LoginForm>, 
                  database: &State<Arc<Mutex<Database>>>) -> Result<Redirect, Flash<Redirect>> {
    if let Ok(database) = database.lock() {
        match database.get_user(UserQuery::Username(&*form_data.into_inner().username)) {  
            Ok(Some(user)) => {
                Ok(Redirect::to("/home"))
            },
            Ok(None) => Err(Flash::error(Redirect::to(uri!("/login")), "Wrong username/password. Please try again!")),
            Err(e) => Err(Flash::error(Redirect::to(uri!("/login")), e.to_string()))
        }
    } else {
        Err(Flash::error(Redirect::to(uri!("/login")), "Failed to login, internal server error. (Failed to acquire lock on database)"))
    }
}