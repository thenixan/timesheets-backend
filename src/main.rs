#![feature(decl_macro)]

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;


use crate::database::TimesheetsDatabaseInitialized;
use crate::routes::TimesheetsRoutesInitialized;

mod routes;
pub mod handlers;
pub mod database;
mod config;


fn main() {
    rocket::custom(config::from_env())
        .manage_database()
        .mount_timesheet_routes()
        .launch();
}
