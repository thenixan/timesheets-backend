use rocket::fairing::AdHoc;
use rocket::Rocket;
use rocket_contrib::database;
use rocket_contrib::databases::diesel;
use rocket_contrib::databases::mongodb;

pub mod authorization;

#[database("user-related")]
pub struct DatabaseConnection(diesel::PgConnection);

#[database("docs-related")]
pub struct DocumentsStorage(mongodb::db::Database);

pub trait TimesheetsDatabaseInitialized {
    fn manage_database(self) -> Self;
}

embed_migrations!("migrations");

impl TimesheetsDatabaseInitialized for Rocket {
    fn manage_database(self) -> Self {
        self.attach(DatabaseConnection::fairing())
            .attach(AdHoc::on_attach("Running migrations", |r| {
                if let Some(c) = DatabaseConnection::get_one(&r) {
                    if let Err(e) = embedded_migrations::run(&*c) {
                        eprint!("Failed to run database migrations: {:?}", e);
                        return Err(r);
                    }
                }
                return Ok(r);
            }))
            .attach(DocumentsStorage::fairing())
    }
}

pub enum RegistrationOutcome {
    Ok,
    AlreadyInUse,
    WeakPassword,
    Other,
}

pub enum AuthorizationOutcome {
    Ok(String),
    NotFound,
    Other,
}
