pub mod route_objects;
mod authentication;
mod routes_setup;

pub trait TimesheetsRoutesInitialized {
    fn mount_timesheet_routes(self) -> Self;
}
