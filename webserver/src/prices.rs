use actix_web::{Error, HttpResponse, Scope, get, web };
use db::establish_connection;
use db::price::StockPrice;

pub fn price_service(path: &str) -> Scope {
    web::scope(path)
        .service(get_prices)
        .service(get_price)
}

#[get("/")]
async fn get_prices() -> Result<HttpResponse, Error> {
    let connection = establish_connection();

    let prices = StockPrice::retrieve_all(&connection);

    Ok(HttpResponse::Ok().json(prices))
}

#[get("/{ticker}")]
async fn get_price(ticker: web::Path<String>) -> Result<HttpResponse, Error> {
    let connection = establish_connection();

    Ok(HttpResponse::Ok().json(StockPrice::retrieve_price(&connection, &ticker.into_inner())))
}
