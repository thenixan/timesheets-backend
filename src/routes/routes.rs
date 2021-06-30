use crate::routes::route_objects::error_response::{ErrorResponse, ERROR_WRONG_REQUEST, ERROR_USER_NOT_FOUND, ERROR_UNKNOWN, ERROR_WEAK_PASSWORD, ERROR_ALREADY_REGISTERED};
use crate::routes::route_objects::login_request::LoginRequest;
use rocket::serde::json::Json;
use crate::handlers::authentication::login::LoginError;
use crate::routes::route_objects::registration_request::RegistrationRequest;
use crate::handlers::authentication::registration::RegistrationError;

#[post("/login", format = "json", data = "<maybe_login_request>")]
pub fn login(maybe_login_request: Option<Json<LoginRequest>>) -> Result<String, Json<&ErrorResponse>> {
    let call_chain = maybe_login_request.map(|r| crate::handlers::authentication::login::login(r.login, r.password));
    return match call_chain {
        Some(Ok(token)) => Ok(token),
        Some(Err(LoginError::NotFound)) => Err(Json(ERROR_USER_NOT_FOUND)),
        None => Err(Json(ERROR_WRONG_REQUEST)),
        _ => Err(Json(ERROR_UNKNOWN)),
    };
}

#[post("/registration", format = "json", data = "<maybe_registration_request>")]
pub fn registration(maybe_registration_request: Option<Json<RegistrationRequest>>) -> Result<(), Json<&ErrorResponse>> {
    let call_chain = maybe_registration_request.map(|r| crate::handlers::authentication::registration::registration(r.login, r.password));
    return match call_chain {
        Some(None) => Ok(()),
        Some(Some(RegistrationError::LoginInUse)) => Err(Json(ERROR_ALREADY_REGISTERED)),
        Some(Some(RegistrationError::WeakPassword)) => Err(Json(ERROR_WEAK_PASSWORD)),
        None => Err(Json(ERROR_WRONG_REQUEST)),
        _ => Err(Json(ERROR_UNKNOWN)),
    };
}