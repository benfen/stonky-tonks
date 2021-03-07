use diesel::SqliteConnection;
use diesel::prelude::*;
use serde::{ Deserialize, Serialize };

use super::schema::stockprice::dsl::*;
use super::schema::stockprice;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct StockPrice {
    pub name: String,
    pub symbol: String,
    pub price: i32,
}

#[derive(Debug, Insertable)]
#[table_name="stockprice"]
pub struct NewStockPrice<'a> {
    pub name: &'a str,
    pub symbol: &'a str,
    pub price: i32,
}

impl StockPrice {
    pub fn retrieve_price(connection: &SqliteConnection, search_symbol: &str) -> Option<StockPrice> {
        stockprice::table.select((name, symbol, price))
            .filter(symbol.eq(search_symbol))
            .first(connection)
            .optional()
            .expect("Error loading user from table")
    }

    pub fn retrieve_all(connection: &SqliteConnection) -> Vec<StockPrice> {
        stockprice::table.select((name, symbol, price))
            .load(connection)
            .expect("Error loading stock prices")
    }
}

impl<'a> NewStockPrice<'a> {
    
    pub fn insert(&self, connection: &SqliteConnection) {
        diesel::insert_into(stockprice::table)
            .values(self)
            .execute(connection)
            .expect("Error saving new stock price");
    }

    pub fn insert_update(&self, connection: &SqliteConnection) {
        let results = stockprice::table.filter(symbol.eq(self.symbol))
            .load::<StockPrice>(connection)
            .expect("Error loading stock prices");

        if results.len() == 0 {
            self.insert(connection);
        } else {
            self.update(connection);
        }
    }

    pub fn update(&self, connection: &SqliteConnection) {
        diesel::update(stockprice::table.filter(symbol.eq(self.symbol)))
            .set(price.eq(self.price))
            .execute(connection)
            .expect("Error updating stock price in db");
    }
}
