use diesel::SqliteConnection;
use diesel::prelude::*;
use serde::{ Deserialize, Serialize };

use super::schema::stockprice::dsl::*;
use super::schema::stockprice;

#[derive(Deserialize, Queryable, Serialize)]
pub struct StockPrice {
    pub name: String,
    pub symbol: String,
    pub price: i32,
}

#[derive(Insertable)]
#[table_name="stockprice"]
pub struct NewStockPrice<'a> {
    pub name: &'a str,
    pub symbol: &'a str,
    pub price: i32,
}

impl StockPrice {
    pub fn retrieve(connection: &SqliteConnection) -> Vec<StockPrice> {
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
