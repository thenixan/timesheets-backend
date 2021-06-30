mod routes_setup;
mod routes;
mod route_objects;

pub trait TimesheetsRoutesInitialized {
    fn mount_timesheet_routes(self) -> Self;
}