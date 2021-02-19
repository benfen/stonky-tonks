mod models;
mod schema;

use uuid::Uuid;
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

use diesel::prelude::*;
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
        let new_stock_price = NewStockPrice{
            id: &Uuid::new_v4().to_hyphenated().to_string(),
            name: &record.name,
            symbol: &record.symbol,
            price: &record.lastsale,
        };
    
        diesel::insert_into(stockprice::table)
            .values(&new_stock_price)
            .execute(&connection)
            .expect("Error saving new post");
    
    });

    Ok(())
}
