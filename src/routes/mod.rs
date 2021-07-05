pub mod route_objects;
mod routes;
mod routes_setup;

pub trait TimesheetsRoutesInitialized {
    fn mount_timesheet_routes(self) -> Self;
}
