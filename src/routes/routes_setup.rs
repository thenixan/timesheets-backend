use rocket::Rocket;

use crate::routes::authentication;
use crate::routes::projects;
use crate::routes::route_objects::error_response::{
    ERROR_UNAUTHORIZED, ERROR_UNKNOWN, ErrorResponse,
};
use crate::routes::TimesheetsRoutesInitialized;

impl TimesheetsRoutesInitialized for Rocket {
    fn mount_timesheet_routes(self) -> Self {
        self.mount(
            "/api-v1",
            routes![
                authentication::login,
                authentication::registration,
                projects::list_projects,
            ],
        )
        .register(catchers![unauthorized, unknown])
    }
}

#[catch(401)]
fn unauthorized<'r>() -> ErrorResponse<'r> {
    return ERROR_UNAUTHORIZED;
}
#[catch(500)]
fn unknown<'r>() -> ErrorResponse<'r> {
    return ERROR_UNKNOWN;
}
