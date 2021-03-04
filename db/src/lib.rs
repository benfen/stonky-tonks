mod stockprice_models;
mod user_models;
mod schema;

pub use stockprice_models::*;
pub use user_models::*;
pub use schema::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}