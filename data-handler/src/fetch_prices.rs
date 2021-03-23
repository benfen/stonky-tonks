use serde::Deserialize;

use db::establish_connection;
use db::price::NewStockPrice;

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
    #[serde(rename = "marketCap")]
    market_cap: String,
    url: String,
}

pub async fn fetch_prices() -> reqwest::Result<()> {
    let body: StockData = reqwest::get(
        "https://api.nasdaq.com/api/screener/stocks?limit=100&tableonly=true&exchange=NASDAQ",
    )
    .await?
    .json()
    .await?;

    let connection = establish_connection();

    body.data.table.rows.into_iter().for_each(|record| {
        let price: i32 = record.lastsale[1..]
            .replace(&['$', '.'][..], "")
            .parse()
            .unwrap();

        let new_stock_price = NewStockPrice {
            name: &record.name,
            symbol: &record.symbol,
            price: price,
        };

        new_stock_price.insert_update(&connection);
    });

    Ok(())
}
