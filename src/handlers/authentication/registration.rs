use crate::database;
use crate::database::{AuthorizationDatabase, RegistrationOutcome};

pub type Token = String;

pub enum RegistrationError {
    LoginInUse,
    WeakPassword,
    Other,
}

pub fn registration(login: &str, password: &str, db: database::Conn) -> Result<(), RegistrationError> {
    match db.registration(login, password) {
        RegistrationOutcome::Ok => Ok(()),
        RegistrationOutcome::AlreadyInUse => Err(RegistrationError::LoginInUse),
        RegistrationOutcome::WeakPassword => Err(RegistrationError::WeakPassword),
        _ => Err(RegistrationError::Other)
    }
}