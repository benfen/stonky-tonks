use uuid::Uuid;
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
    pub quantity: i32,
}

#[derive(Debug, Identifiable, Insertable)]
#[table_name="stockholdings"]
pub struct ModStockHoldings<'a> {
    id: String,
    pub userid: &'a str,
    pub stockid: &'a str,
    pub quantity: i32,
}

impl StockHolding {
    pub fn retrieve_holdings(connection: &SqliteConnection, user: &User) -> Vec<Self> {
        StockHolding::belonging_to(user)
            .load::<StockHolding>(connection)
            .expect("Error loading user from table")
    }
}

impl<'a> ModStockHoldings<'a> {

    pub fn create_new_holding(user: &'a User, ticker: &'a str, quantity: i32, connection: &SqliteConnection) -> Self {
        let new_id = Uuid::new_v4().to_hyphenated().to_string();
        
        let new_holding = ModStockHoldings {
            id: new_id,
            userid: &user.id,
            stockid: ticker,
            quantity,
        };

        diesel::insert_into(stockholdings::table)
            .values(&new_holding)
            .execute(connection)
            .expect("Error creating stock holding");

        new_holding
    }

    pub fn update_quantity(&self, new_quantity: i32, connection: &SqliteConnection) {
        use super::schema::stockholdings::dsl::*;

        diesel::update(self).set(
                quantity.eq(new_quantity)
            )
            .execute(connection)
            .expect("Error updating quantity for stock holding");
    }
}
