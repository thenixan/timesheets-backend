use crate::database::{AuthorizationDatabase, Conn, AuthorizationOutcome};
use crate::schema::users;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl};
use uuid::Uuid;

#[derive(Queryable, PartialEq, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub secret: String,
}

impl AuthorizationDatabase for Conn {
    fn login(&self, login: &str, password: &str) -> AuthorizationOutcome {
        match users::table.filter(users::username.eq(login.to_lowercase())).get_result::<User>(&self.0) {
            Ok(user) => {
                if user.secret == password {
                    AuthorizationOutcome::Ok(user.id.to_string())
                } else {
                    AuthorizationOutcome::NotFound
                }
            }
            Err(diesel::result::Error::NotFound) => AuthorizationOutcome::NotFound,
            _ => AuthorizationOutcome::Other,
        }
    }
}