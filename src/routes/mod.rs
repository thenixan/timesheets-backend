mod authentication;
pub mod guards;
mod projects;
pub mod route_objects;
mod routes_setup;

pub trait TimesheetsRoutesInitialized {
    fn mount_timesheet_routes(self) -> Self;
}
