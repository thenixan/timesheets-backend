use crate::database::{AuthorizationDatabase, Conn, AuthorizationOutcome, RegistrationOutcome};
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

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub secret: &'a str,
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

    fn registration(&self, login: &str, password: &str) -> RegistrationOutcome {
        if password.len() < 8 {
            return RegistrationOutcome::WeakPassword;
        }
        let new_user = NewUser { username: login, secret: password };
        return match diesel::insert_into(users::table).values(new_user).get_result::<User>(&self.0) {
            Ok(_) => RegistrationOutcome::Ok,
            Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => RegistrationOutcome::AlreadyInUse,
            _ => RegistrationOutcome::Other,
        };
    }
}