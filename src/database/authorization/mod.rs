use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use rand_core::OsRng;
use uuid::Uuid;

use crate::database::RegistrationOutcome;
use crate::schema::users;

pub mod get_user;
pub mod token;

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

pub fn save_credentials(
    db: &diesel::PgConnection,
    login: &str,
    password: &str,
) -> RegistrationOutcome {
    if password.len() < 8 {
        return RegistrationOutcome::WeakPassword;
    }
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password_simple(password.as_bytes(), salt.as_ref())
        .unwrap();
    let new_user = NewUser {
        username: login,
        secret: &password_hash.to_string(),
    };
    return match diesel::insert_into(users::table)
        .values(new_user)
        .get_result::<User>(db)
    {
        Ok(_) => RegistrationOutcome::Ok,
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => RegistrationOutcome::AlreadyInUse,
        _ => RegistrationOutcome::Other,
    };
}
