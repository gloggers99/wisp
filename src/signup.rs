use std::sync::{Arc, Mutex};

use maud::{html, Markup};

use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::State;

use wisp_database::user::User;
use wisp_database::database::Database;

#[derive(FromForm)]
pub struct SignupForm {
    email: String,
    username: String,
    password: String
} impl Into<User> for SignupForm {
    fn into(self) -> User {
        User::new(self.email, self.username, self.password, None)
    }
}

#[get("/signup")]
pub fn signup_get(flash: Option<FlashMessage>) -> Markup {
    html!(
        (maud::DOCTYPE)
        body {
            @if let Some(flash) = flash {
                p style="color:red; font-weight: bold;" { (flash.message()) }
            }
            form method="post" action="/signup" {
                p {}
                label for="email" { "Email:" }
                input type="text" name="email" id="email" required;
                br;
                label for="username" { "Username:" }
                input type="text" name="username" id="username" required;
                br;
                label for="password" { "Password:" }
                input type="password" name="password" id="password" required;
                br;
                button type="submit" { "Sign Up" }
            }
        }
    )
}

#[post("/signup", data="<signup_form>")]
pub fn signup_post(signup_form: Form<SignupForm>,
                   database: &State<Arc<Mutex<Database>>>) -> Result<Redirect, Flash<Redirect>> {
    // `if let` here to show when the database is clearly locked.
    if let Ok(database) = database.lock() {
        match database.add_user(signup_form.into_inner().into()) {
            Ok(()) => {
                Ok(Redirect::to(uri!("/login")))
            },
            // Failed to sign up.
            Err(e) => {
                Err(Flash::error(Redirect::to(uri!("/signup")), e.to_string()))
            }
        }
    } else {
        Err(Flash::error(Redirect::to(uri!("/signup")), "Internal server error. (Failed to acquire lock on database)"))
    }
}