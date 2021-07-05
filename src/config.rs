use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use rocket::config::{Environment, Value};
use rocket::Config;

pub fn from_env() -> Config {
    dotenv().ok();
    let environment = Environment::active().expect("No environment found");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut databases = HashMap::new();

    let mut database_config = HashMap::new();
    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", Value::from(database_url));
    databases.insert("user-related", Value::from(database_config));

    let mut mongo_config = HashMap::new();
    let mongo_url = env::var("MONGO_URL").expect("No MONGO_URL environment variable found");
    mongo_config.insert("url", Value::from(mongo_url));
    databases.insert("docs-related", Value::from(mongo_config));

    Config::build(environment)
        .environment(environment)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
