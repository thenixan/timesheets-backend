mod routes;
pub mod handlers;

#[macro_use]
extern crate rocket;

use crate::routes::TimesheetsRoutesInitialized;


#[launch]
fn rocket() -> _ {
    rocket::build().mount_timesheet_routes()
}