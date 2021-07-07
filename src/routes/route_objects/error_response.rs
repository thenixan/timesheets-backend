use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Result};
use rocket::{Request, Response};
use rocket_contrib::json::Json;

#[derive(Copy, Clone, Debug)]
pub struct ErrorResponse<'a> {
    cause: &'a str,
    pub status: Status,
}

impl<'r> Responder<'r> for ErrorResponse<'r> {
    fn respond_to(self, request: &Request) -> Result<'r> {
        if let Ok(response) = Json(json!({"error": self.cause})).respond_to(request) {
            Response::build_from(response)
                .status(self.status)
                .header(ContentType::JSON)
                .ok()
        } else {
            Response::build()
                .status(Status::InternalServerError)
                .header(ContentType::JSON)
                .ok()
        }
    }
}

// common errors
pub const ERROR_UNKNOWN: ErrorResponse = ErrorResponse {
    cause: "unknown",
    status: Status::InternalServerError,
};
pub const ERROR_WRONG_REQUEST: ErrorResponse = ErrorResponse {
    cause: "wrong_request",
    status: Status::BadRequest,
};
pub const ERROR_UNAUTHORIZED: ErrorResponse = ErrorResponse {
    cause: "unauthorized",
    status: Status::Unauthorized,
};

// login error
pub const ERROR_USER_NOT_FOUND: ErrorResponse = ErrorResponse {
    cause: "user_not_found",
    status: Status::BadRequest,
};

// registration error
pub const ERROR_WEAK_PASSWORD: ErrorResponse = ErrorResponse {
    cause: "weak_password",
    status: Status::BadRequest,
};
pub const ERROR_ALREADY_REGISTERED: ErrorResponse = ErrorResponse {
    cause: "already_registered",
    status: Status::BadRequest,
};
