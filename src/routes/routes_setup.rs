use rocket::{Rocket, Build};
use crate::routes::TimesheetsRoutesInitialized;
use crate::routes::routes;

impl TimesheetsRoutesInitialized for Rocket<Build> {
    fn mount_timesheet_routes(self) -> Self {
        self.mount("/api-v1", routes![
            routes::login,
            routes::registration
        ])
    }
}