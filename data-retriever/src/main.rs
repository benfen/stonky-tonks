mod models;
mod schema;

use crate::models::StockPrice;
use serde::{Deserialize};
use self::diesel::prelude::*;

use models::NewStockPrice;
use schema::stockprice;

#[derive(Debug, Deserialize)]
struct StockData {
    data: StockTable,
}

#[derive(Debug, Deserialize)]
struct StockTable {
    table: StockRows,
}

#[derive(Debug, Deserialize)]
struct StockRows {
    rows: Vec<Row>,
}

#[derive(Debug, Deserialize)]
struct Row {
    symbol: String,
    name: String,
    lastsale: String,
    netchange: String,
    pctchange: String,
    marketCap: String,
    url: String
}

#[macro_use]
extern crate diesel;
extern crate dotenv;

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

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let body: StockData = reqwest::get("https://api.nasdaq.com/api/screener/stocks?limit=100&tableonly=true&exchange=NASDAQ")
    .await?
    .json()
    .await?;

    let connection = establish_connection();

    body.data.table.rows.into_iter().for_each(|record| {
        let results = stockprice::table.filter(schema::stockprice::dsl::symbol.eq(&record.symbol))
            .load::<StockPrice>(&connection)
            .expect("Error loading stock prices");
        
        if results.len() == 0 {
            let new_stock_price = NewStockPrice{
                name: &record.name,
                symbol: &record.symbol,
                price: &record.lastsale,
            };
        
            diesel::insert_into(stockprice::table)
                .values(&new_stock_price)
                .execute(&connection)
                .expect("Error saving new stock price");

        } else {
            diesel::update(stockprice::table.filter(schema::stockprice::dsl::symbol.eq(&record.symbol)))
                .set(schema::stockprice::dsl::price.eq(&record.lastsale))
                .execute(&connection)
                .expect("Error updating stock price in db");
        }
    
    });

    Ok(())
}
