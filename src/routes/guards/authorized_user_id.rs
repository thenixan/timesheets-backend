use crate::database::authorization::get_user::GetUserOutcome;
use crate::database::DatabaseConnection;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use uuid::Uuid;

pub struct AuthorizedUser {
    pub user_id: Uuid,
}

impl<'r, 'a> FromRequest<'r, 'a> for AuthorizedUser {
    type Error = ();

    fn from_request(request: &'r Request<'a>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("authorization");
        return if let Some(auth_string) = auth_header {
            if let Outcome::Success(conn) = request.guard::<DatabaseConnection>() {
                match crate::database::authorization::get_user::with_token(&*conn, auth_string) {
                    GetUserOutcome::Some(user) => {
                        Outcome::Success(AuthorizedUser { user_id: user.id })
                    }
                    _ => Outcome::Failure((Status::Unauthorized, ())),
                }
            } else {
                Outcome::Failure((Status::InternalServerError, ()))
            }
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        };
    }
}
