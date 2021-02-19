use diesel::types::Integer;
use super::schema::stockprice;

#[derive(Queryable)]
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
