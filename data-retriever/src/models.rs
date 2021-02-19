use super::schema::stockprice;

#[derive(Queryable)]
pub struct StockPrice {
    pub name: String,
    pub symbol: String,
    pub price: String,
}

#[derive(Insertable)]
#[table_name="stockprice"]
pub struct NewStockPrice<'a> {
    pub name: &'a str,
    pub symbol: &'a str,
    pub price: &'a str,
}
