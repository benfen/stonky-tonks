use diesel::SqliteConnection;
use diesel::prelude::*;
use serde::{ Deserialize, Serialize };

use crate::user_models::User;
use crate::stockprice_models::StockPrice;

use super::schema::stockholdings;
use super::schema::stockprice;

#[derive(Associations, Debug, Deserialize, Identifiable, Queryable, Serialize)]
#[table_name="stockholdings"]
#[belongs_to(User, foreign_key="userid")]
#[primary_key(userid, stockid)]
pub struct StockHolding {
    pub userid: String,
    pub stockid: String,
    quantity: i32,
}

#[derive(Debug, Identifiable, Insertable)]
#[table_name="stockholdings"]
#[primary_key(userid, stockid)]
pub struct ModStockHoldings<'a> {
    pub userid: &'a str,
    pub stockid: &'a str,
    quantity: i32,
}

impl StockHolding {
    pub fn retrieve_all_holdings(connection: &SqliteConnection, user: &User) -> Vec<Self> {
        StockHolding::belonging_to(user)
            .load::<StockHolding>(connection)
            .expect("Error loading holdings from table")
    }

    pub fn retrieve_all_holdings_with_price(connection: &SqliteConnection, user: &User) -> Vec<(Self, StockPrice)> {
        use super::schema::stockholdings::dsl::userid;

        stockholdings::table.inner_join(stockprice::table)
            .filter(userid.eq(&user.id))
            .load(connection)
            .expect("Error loading holdings from table")
    }

    pub fn retrieve_holding(connection: &SqliteConnection, user: &User, ticker: &str) -> Option<Self> {
        use super::schema::stockholdings::dsl::stockholdings;

        stockholdings.find((&user.id, ticker))
            .first(connection)
            .optional()
            .expect("Error retrieving holding")
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity as u32
    }
}

impl<'a> ModStockHoldings<'a> {

    pub fn create_new_holding(user: &'a User, ticker: &'a str, quantity: u32, connection: &SqliteConnection) -> ModStockHoldings<'a> {        
        let new_holding = ModStockHoldings {
            userid: &user.id,
            stockid: ticker,
            quantity: quantity as i32,
        };

        diesel::insert_into(stockholdings::table)
            .values(&new_holding)
            .execute(connection)
            .expect("Error creating stock holding");

        new_holding
    }

    fn liquidate_holding(holding: &StockHolding, connection: &SqliteConnection) {
        diesel::delete(holding)
            .execute(connection)
            .expect("Error deleting holding in db");
    }

    pub fn update_quantity(holding: &StockHolding, new_quantity: u32, connection: &SqliteConnection) {
        use super::schema::stockholdings::dsl::*;

        if new_quantity == 0 {
            ModStockHoldings::liquidate_holding(holding, connection);
        } else {
            diesel::update(holding).set(
                    quantity.eq(new_quantity as i32)
                )
                .execute(connection)
                .expect("Error updating quantity for stock holding");
        }
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity as u32
    }
}
