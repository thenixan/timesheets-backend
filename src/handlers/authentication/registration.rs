use crate::database;
use crate::database::RegistrationOutcome;

pub type Token = String;

pub enum RegistrationError {
    LoginInUse,
    WeakPassword,
    Other,
}

pub fn registration(
    login: &str,
    password: &str,
    db: database::DatabaseConnection,
) -> Result<(), RegistrationError> {
    match database::authorization::registration(&*db, login, password) {
        RegistrationOutcome::Ok => Ok(()),
        RegistrationOutcome::AlreadyInUse => Err(RegistrationError::LoginInUse),
        RegistrationOutcome::WeakPassword => Err(RegistrationError::WeakPassword),
        _ => Err(RegistrationError::Other),
    }
}
