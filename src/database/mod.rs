use rocket::Rocket;
use rocket_contrib::databases::diesel;
use rocket::fairing::AdHoc;

#[database("diesel_postgres_pool")]
pub struct Conn(diesel::PgConnection);


pub trait TimesheetsDatabaseInitialized {
    fn manage_database(self) -> Self;
}

embed_migrations!("migrations");

impl TimesheetsDatabaseInitialized for Rocket {
    fn manage_database(self) -> Self {
        self.attach(Conn::fairing())
            .attach(AdHoc::on_attach("Running migrations", |r| {
                if let Some(c) = Conn::get_one(&r) {
                    if let Err(e) = embedded_migrations::run(&*c) {
                        eprint!("Failed to run database migrations: {:?}", e);
                        return Err(r);
                    }
                }
                return Ok(r);
            }))
    }
}