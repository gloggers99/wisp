use std::sync::{Arc, Mutex};

use maud::{Markup};

use rocket::form::Form;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::State;

use wisp_database::user::User;
use wisp_database::database::Database;

use wisp_pages::signup::signup_page;

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
    if let Some(flash) = flash {
        signup_page(Some(flash.message()))
    } else {
        signup_page(None)
    }
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