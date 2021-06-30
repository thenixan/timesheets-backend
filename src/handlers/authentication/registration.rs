pub type Token = String;

pub enum RegistrationError {
    LoginInUse,
    WeakPassword,
    Other,
}

pub fn registration(login: &str, password: &str) -> Option<RegistrationError> {
    //TODO changeme
    if login == "thenixan" {
        Some(RegistrationError::LoginInUse)
    } else if password.len() < 8 {
        Some(RegistrationError::WeakPassword)
    } else {
        None
    }
}