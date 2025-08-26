use crate::session_manager::{SessionManager, SESSION_LENGTH};

use std::str::FromStr;
use std::sync::{Arc, Mutex};

use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

use uuid::Uuid;

use wisp_database::database::{Database, UserQuery};
use wisp_database::user::User;

pub struct AuthenticatedUser {
    pub user: User
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let database = request.rocket().state::<Arc<Mutex<Database>>>().unwrap();
        let session_manager = request.rocket().state::<Arc<Mutex<SessionManager>>>().unwrap();
        let cookies = request.cookies();

        if let Some(cookie) = cookies.get_private("session_id")
            && let Ok(uuid) = Uuid::from_str(cookie.value_trimmed())
            && let Ok(session_manager) = session_manager.lock()
            && let Some((session, username)) = session_manager.get_session_by_uuid(uuid) {

            if session.created().elapsed().unwrap() > SESSION_LENGTH {
                return Outcome::Forward(Status::Forbidden)
            }

            if let Ok(database) = database.lock() {
                if let Ok(Some(user)) = database.get_user(UserQuery::Username(&username)) {
                    return Outcome::Success(AuthenticatedUser { user });
                }
            }
        }

        Outcome::Forward(Status::Forbidden)
    }
}
