use super::schema::stockprice;

#[derive(Queryable)]
pub struct StockPrice {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub price: String,
}

#[derive(Insertable)]
#[table_name="stockprice"]
pub struct NewStockPrice<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub symbol: &'a str,
    pub price: &'a str,
}
