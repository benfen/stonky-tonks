// #[macro_use]
// extern crate diesel;

use serde::{Deserialize};

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

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let body: StockData = reqwest::get("https://api.nasdaq.com/api/screener/stocks?limit=100&tableonly=true&exchange=NASDAQ")
    .await?
    .json()
    .await?;

    println!("{:?}", body.data.table.rows[0]);

    Ok(())
}
