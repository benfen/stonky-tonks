use diesel::SqliteConnection;
use diesel::prelude::*;
use serde::{ Deserialize, Serialize };

use crate::user_models::User;

use super::schema::stockholdings;

#[derive(Associations, Debug, Deserialize, Identifiable, Queryable, Serialize)]
#[table_name="stockholdings"]
#[belongs_to(User, foreign_key="userid")]
pub struct StockHolding {
    pub id: String,
    pub userid: String,
    pub stockid: String,
}

impl StockHolding {
    pub fn retrieve_holdings(connection: &SqliteConnection, user: &User) -> Vec<Self> {
        StockHolding::belonging_to(user)
            .load::<StockHolding>(connection)
            .expect("Error loading user from table")
    }
}
