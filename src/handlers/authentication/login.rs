use crate::database;
use crate::handlers::authentication::login::LoginError::{NotFound, Other};
use crate::database::{AuthorizationDatabase, AuthorizationOutcome};
use crate::handlers::authentication::registration::RegistrationError::LoginInUse;

pub type Token = String;

pub enum LoginError {
    NotFound,
    Other,
}

pub fn login(login: &str, password: &str, db: database::Conn) -> Result<Token, LoginError> {
    match db.login(login, password) {
        AuthorizationOutcome::Ok(s) => Ok(s),
        AuthorizationOutcome::NotFound => Err(LoginError::NotFound),
        AuthorizationOutcome::Other => Err(LoginError::Other),
    }
}