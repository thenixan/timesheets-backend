use rocket::Rocket;

use crate::routes::routes;
use crate::routes::TimesheetsRoutesInitialized;

impl TimesheetsRoutesInitialized for Rocket {
    fn mount_timesheet_routes(self) -> Self {
        self.mount("/api-v1", routes![routes::login, routes::registration])
    }
}
