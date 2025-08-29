use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::authenticated_user::AuthenticatedUser;

pub struct AdminUser {
    authenticated_user: AuthenticatedUser
} impl AdminUser {
    pub fn authenticated_user(&self) -> &AuthenticatedUser {
        &self.authenticated_user
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = ();
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match AuthenticatedUser::from_request(request).await { 
            Outcome::Success(authenticated_user) => {
                if authenticated_user.user.username() == "lucas" {
                    Outcome::Success(AdminUser {
                        authenticated_user
                    })
                } else {
                    Outcome::Forward(Status::Forbidden)
                }
            },
            Outcome::Forward(status) => Outcome::Forward(status),
            Outcome::Error(err) => Outcome::Error(err)
        }
    }
}