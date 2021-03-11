mod holdings_models;
mod stockprice_models;
mod user_models;
mod schema;

pub mod price {
    pub use crate::stockprice_models::*;
}

pub mod user {
    pub use crate::user_models::*;
}

pub mod holdings {
    pub use crate::holdings_models::*;
}

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::result::Error;
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

pub fn perform_transation<T, F>(connection: &SqliteConnection, f: F) -> Result<T, Error>
        where F: FnOnce() -> Result<T, Error> {
    connection.transaction(f)
}