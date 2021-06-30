use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse<'a> {
    pub message: &'a str,
    pub code: i16,
}

// common errors
pub const ERROR_UNKNOWN: &'static ErrorResponse<'static> = &ErrorResponse { message: "something went wrong", code: 500 };
pub const ERROR_WRONG_REQUEST: &'static ErrorResponse<'static> = &ErrorResponse { message: "wrong request", code: 501 };

// login error
pub const ERROR_USER_NOT_FOUND: &'static ErrorResponse<'static> = &ErrorResponse { message: "user not found", code: 200 };

// registration error
pub const ERROR_WEAK_PASSWORD: &'static ErrorResponse<'static> = &ErrorResponse { message: "weak password", code: 210 };
pub const ERROR_ALREADY_REGISTERED: &'static ErrorResponse<'static> = &ErrorResponse { message: "already registered", code: 211 };