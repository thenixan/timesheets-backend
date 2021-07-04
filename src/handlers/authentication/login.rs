use crate::database;
use crate::database::AuthorizationOutcome;

pub type Token = String;

pub enum LoginError {
    NotFound,
    Other,
}

pub fn login(login: &str, password: &str, db: database::Conn) -> Result<Token, LoginError> {
    match database::authorization::login(&*db, login, password) {
        AuthorizationOutcome::Ok(s) => Ok(s),
        AuthorizationOutcome::NotFound => Err(LoginError::NotFound),
        AuthorizationOutcome::Other => Err(LoginError::Other),
    }
}
