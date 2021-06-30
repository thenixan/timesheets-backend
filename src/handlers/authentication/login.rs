use crate::handlers::authentication::login::LoginError::{NotFound, Other};

pub type Token = String;

pub enum LoginError {
    NotFound,
    Other,
}

pub fn login(login: &str, password: &str) -> Result<Token, LoginError> {
    //TODO changeme
    if login == "thenixan" && password == "123" {
        Ok("ok".to_string())
    } else if login == "fail" {
        Err(Other)
    } else {
        Err(NotFound)
    }
}