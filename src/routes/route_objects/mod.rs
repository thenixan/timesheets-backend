use rocket::Request;
use rocket::response::{Responder, Result};

use crate::routes::route_objects::error_response::ErrorResponse;

pub mod login_request;
pub mod error_response;
pub mod registration_request;


pub enum ApiResponse<'a, T> {
    Ok(T),
    Err(&'a ErrorResponse<'a>),
}


impl<'r, 'a, T> Responder<'r> for ApiResponse<'r, T> where T: Responder<'r> {
    fn respond_to(self, request: &Request) -> Result<'r> {
        match self {
            ApiResponse::Ok(t) => t.respond_to(request),
            ApiResponse::Err(e) => e.respond_to(request)
        }
    }
}