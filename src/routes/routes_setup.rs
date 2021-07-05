use rocket::Rocket;

use crate::routes::authentication;
use crate::routes::TimesheetsRoutesInitialized;

impl TimesheetsRoutesInitialized for Rocket {
    fn mount_timesheet_routes(self) -> Self {
        self.mount("/api-v1", routes![authentication::login, authentication::registration])
    }
}
