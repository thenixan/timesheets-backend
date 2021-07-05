use rocket_contrib::json::Json;

use crate::database;
use crate::handlers::authentication;
use crate::handlers::authentication::login::LoginError;
use crate::handlers::authentication::registration::RegistrationError;
use crate::routes::route_objects::error_response::{
    ERROR_ALREADY_REGISTERED, ERROR_UNKNOWN, ERROR_USER_NOT_FOUND, ERROR_WEAK_PASSWORD,
    ERROR_WRONG_REQUEST, ErrorResponse,
};
use crate::routes::route_objects::login_request::LoginRequest;
use crate::routes::route_objects::login_response::LoginResponse;
use crate::routes::route_objects::registration_request::RegistrationRequest;

#[post("/login", format = "json", data = "<maybe_login_request>")]
pub fn login<'r>(
    maybe_login_request: Option<Json<LoginRequest>>,
    db: database::DatabaseConnection,
) -> Result<Json<LoginResponse>, ErrorResponse<'r>> {
    let call_chain =
        maybe_login_request.map(|r| authentication::login::login(r.login, r.password, db));
    return match call_chain {
        Some(Ok(token)) => {
            let login_response = LoginResponse::from(token);
            let json_response = Json(login_response);
            Result::Ok(json_response)
        }
        Some(Err(LoginError::NotFound)) => Result::Err(ERROR_USER_NOT_FOUND),
        None => Result::Err(ERROR_WRONG_REQUEST),
        _ => Result::Err(ERROR_UNKNOWN),
    };
}

#[post(
    "/registration",
    format = "json",
    data = "<maybe_registration_request>"
)]
pub fn registration<'r>(
    maybe_registration_request: Option<Json<RegistrationRequest>>,
    db: database::DatabaseConnection,
) -> Result<(), ErrorResponse<'r>> {
    let call_chain = maybe_registration_request
        .map(|r| authentication::registration::registration(&r.login, &r.password, db));
    return match call_chain {
        Some(Ok(_)) => Result::Ok(()),
        Some(Err(RegistrationError::LoginInUse)) => Result::Err(ERROR_ALREADY_REGISTERED),
        Some(Err(RegistrationError::WeakPassword)) => Result::Err(ERROR_WEAK_PASSWORD),
        None => Result::Err(ERROR_WRONG_REQUEST),
        _ => Result::Err(ERROR_UNKNOWN),
    };
}
