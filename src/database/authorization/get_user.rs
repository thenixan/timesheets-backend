use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};

use crate::database::authorization::User;
use crate::schema::tokens;
use crate::schema::users;

pub enum GetUserOutcome {
    Some(User),
    None,
    Error,
}

pub fn with_token(db: &diesel::PgConnection, token: &str) -> GetUserOutcome {
    match users::table
        .left_join(tokens::table.on(tokens::user_id.eq(users::id)))
        .select((users::id, users::username, users::secret))
        .filter(tokens::token.eq(token))
        .get_result::<User>(db)
    {
        Ok(user) => GetUserOutcome::Some(user),
        Err(diesel::result::Error::NotFound) => GetUserOutcome::None,
        _ => GetUserOutcome::Error,
    }
}

pub fn with_credentials(db: &diesel::PgConnection, login: &str, password: &str) -> GetUserOutcome {
    match users::table
        .filter(users::username.eq(login.to_lowercase()))
        .get_result::<User>(db)
    {
        Ok(user) => {
            let argon2 = Argon2::default();
            if let Ok(parsed_hash) = PasswordHash::new(&user.secret) {
                if argon2
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok()
                {
                    GetUserOutcome::Some(user)
                } else {
                    GetUserOutcome::None
                }
            } else {
                GetUserOutcome::None
            }
        }
        Err(diesel::result::Error::NotFound) => GetUserOutcome::None,
        _ => GetUserOutcome::Error,
    }
}
