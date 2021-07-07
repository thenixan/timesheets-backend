use crate::database;
use crate::database::authorization::get_user::GetUserOutcome;
use crate::database::authorization::token::CreateTokenOutcome;

pub enum LoginError {
    NotFound,
    Other,
}

pub fn create_token(
    login: &str,
    password: &str,
    db: database::DatabaseConnection,
) -> Result<String, LoginError> {
    match database::authorization::get_user::with_credentials(&*db, login, password) {
        GetUserOutcome::Some(user) => {
            match database::authorization::token::create_for_user(&*db, &user) {
                CreateTokenOutcome::Ok(token) => Ok(token),
                CreateTokenOutcome::Err => Err(LoginError::Other),
            }
        }
        GetUserOutcome::None => Err(LoginError::NotFound),
        GetUserOutcome::Error => Err(LoginError::Other),
    }
}
